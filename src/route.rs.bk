use futures;
use futures::future::Future;
use futures::BoxFuture;

use regex::{Regex, Captures};

use hyper;
use hyper::StatusCode;
use hyper::header::ContentType;

use handlebars::Handlebars;

use std::sync::Arc;

use request::Request;
use response::Response;
use error::Error;
use status::Status;

pub enum Route {
    Literal {
        path: String,
        handler: fn(Request) -> BoxFuture<Response, Error>,
    },
    Regex {
        regex: Regex,
        handler: fn(&Request, Captures) -> BoxFuture<Response, Error>,
    },
    CatchAll {
        handler: fn(Request) -> BoxFuture<Response, Error>,
    }
}

impl Route {
    pub fn matches(&self, path: &str) -> bool {
        match self {
            &Route::Literal { path: ref p, .. } => {
                p == path
            },
            &Route::Regex { ref regex, .. } => {
                regex.is_match(path)
            },
            &Route::CatchAll { .. } => {
                true
            }
        }
    }

    fn handle(&self, req: Request) -> BoxFuture<Response, Error> {
        match self {
            &Route::Literal { handler, .. } => {
                handler(req)
            },
            &Route::Regex { handler, ref regex } => {
                // i am extremely suspicous of this unwrap
                let captures = regex.captures(req.request.path()).unwrap();

                handler(&req, captures)
            },
            &Route::CatchAll { handler } => {
                handler(req)
            },
        }
    }
    pub fn render_with(&self, req: hyper::server::Request, handlebars: Arc<Handlebars>) -> BoxFuture<hyper::server::Response, hyper::Error> {
        let r = Request {
            request: req,
        };
        self.handle(r).and_then(move |response| {
            match response.status {
                Status::Ok=> {
                    let body = handlebars.render(&response.template, &response.data).unwrap();

                    futures::future::ok(hyper::server::Response::new()
                        .with_header(ContentType::html())
                        .with_body(body))
                }
                Status::NotFound => {
                    ::futures::future::ok(hyper::server::Response::new().with_status(StatusCode::NotFound))
                }
            }
        })
        .map_err(|e| e.inner)
        .boxed()
    }
}
