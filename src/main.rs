#![feature(async_await, await_macro, futures_api, pin)]

#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use simple_server::{Method, StatusCode};

struct Server;

impl Server {
    async fn hello(&self, mut response: simple_server::Builder) -> simple_server::ResponseResult {
        Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec()).unwrap())
    }

    async fn four_oh_four(&self, mut response: simple_server::Builder) -> simple_server::ResponseResult {
        response.status(StatusCode::NOT_FOUND);
        Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
    }

    fn listen(self, host: &str, port: &str) -> ! {
        let server = simple_server::Server::new(move |request, response| {
            println!("Request received. {} {}", request.method(), request.uri());

            // futures::executor::block_on(route(&self, request, response))
            futures::executor::block_on(async {
                match (request.method(), request.uri().path()) {
                    (&Method::GET, "/hello") => {
                        await!(self.hello(response))
                    }
                    (_, _) => {
                        await!(self.four_oh_four(response))
                    }
                }
            })
        });

        server.listen(host, port);
    }
}

fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    let server = Server; 

    server.listen(host, port);
}