use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::net::SocketAddr;

use slog;
use slog::DrainExt;

use slog_term;

use regex::{Regex, Captures};

use futures;
use futures::future::Future;
use futures::BoxFuture;

use futures_cpupool::CpuPool;

use hyper;
use hyper::StatusCode;
use hyper::header::{ContentType, Host, Location};
use hyper::server::{Http, Service};

use handlebars::Handlebars;

use route::Route;
use error::Error;
use response::Response;
use request::Request;

pub struct Server {
    routes: Vec<Route>,
    catch_all_route: Option<Route>,
    log: slog::Logger,
    pool: CpuPool,
    handlebars: Arc<Handlebars>,
}

impl Server {
    /// Construct a `Server`.
    ///
    /// Use a path name to create a Server.  The path holds template files used as a view.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn run () -> Result <(), sparkles::Error> {
    /// let mut server = sparkles::Server::new("templates");
    /// # }
    /// ```
    ///
    pub fn new<S: Into<String>>(template_root: S) -> Server {
        let mut handlebars = Handlebars::new();

        for entry in fs::read_dir(&template_root.into()).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_stem().unwrap().to_str().unwrap();

            handlebars
                .register_template_file(name, &path)
                .ok()
                .unwrap();
        }

        Server {
            routes: Vec::new(),
            catch_all_route: None,
            log: slog::Logger::root(slog_term::streamer().full().build().fuse(), o!()),
            pool: CpuPool::new(4), // FIXME: is this right? who knows!
            handlebars: Arc::new(handlebars),
        }
    }

    pub fn add_route(&mut self, path: &str, handler: fn(Request) -> BoxFuture<Response, Error>) {
        let path = path.to_string();

        self.routes
            .push(
                Route::Literal {
                    path: path,
                    handler: handler,
                },
            );
    }

    pub fn add_regex_route(
        &mut self,
        regex: &str,
        handler: fn(&Request, Captures) -> BoxFuture<Response, Error>,
    ) {
        self.routes
            .push(
                Route::Regex {
                    regex: Regex::new(regex).unwrap(),
                    handler: handler,
                },
            );
    }

    pub fn add_catch_all_route(&mut self, handler: fn(Request) -> BoxFuture<Response, Error>) {
        self.catch_all_route = Some(Route::CatchAll { handler: handler });
    }

    pub fn run(self, addr: &SocketAddr) {
        self.run_until(addr, futures::future::empty());
    }

    pub fn run_until<F>(self, addr: &SocketAddr, shutdown_signal: F) where F: Future<Item=(), Error=()> {
        info!(self.log, "Starting server, listening on http://{}", addr);

        let a = Arc::new(self);

        let server = Http::new().bind(addr, move || Ok(a.clone())).unwrap();

        server.run_until(shutdown_signal).unwrap();
    }
}

impl Service for Server {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: hyper::server::Request) -> Self::Future {
        // redirect to ssl
        // from http://jaketrent.com/post/https-redirect-node-heroku/
        if let Some(raw) = req.headers().get_raw("x-forwarded-proto") {
            if raw != &b"https"[..] {
                let host: &Host = req.headers().get().unwrap();
                return ::futures::future::ok(
                    hyper::server::Response::new()
                    .with_header(Location::new(format!("https://{}{}",
                         host,
                         req.path())))
                    .with_status(StatusCode::MovedPermanently)
                ).boxed();
            }
        }

        // first, we serve static files
        let fs_path = format!("public{}", req.path());

        // ... you trying to do something bad?
        if fs_path.contains("./") || fs_path.contains("../") {
            // GET OUT
            return ::futures::future::ok(
                hyper::server::Response::new()
                    .with_header(ContentType::html())
                    .with_status(StatusCode::NotFound),
            )
                           .boxed();
        }

        if Path::new(&fs_path).is_file() {
            return self.pool
                       .spawn_fn(
                move || {
                    let mut f = File::open(&fs_path).unwrap();

                    let mut source = Vec::new();

                    f.read_to_end(&mut source).unwrap();

                    futures::future::ok(hyper::server::Response::new().with_body(source))
                },
            )
                       .boxed();
        }

        // next, we check routes

        for route in &self.routes {
            if route.matches(req.path()) {
                let handlebars = self.handlebars.clone();
                return route.render_with(req, handlebars);
            }
        }

        if let Some(ref h) = self.catch_all_route {
            let handlebars = self.handlebars.clone();
            return h.render_with(req, handlebars);
        }

        ::futures::future::ok(
            hyper::server::Response::new()
                .with_header(ContentType::html())
                .with_status(StatusCode::NotFound),
        )
                .boxed()
    }
}
