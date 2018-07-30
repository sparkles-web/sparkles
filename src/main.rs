#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use simple_server::{Method, StatusCode};

struct Server;

impl Server {
    fn hello(&self, mut response: simple_server::Builder) -> simple_server::ResponseResult {
        Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())?)
    }

    fn four_oh_four(&self, mut response: simple_server::Builder) -> simple_server::ResponseResult {
        response.status(StatusCode::NOT_FOUND);
        Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
    }

    fn listen(self, host: &str, port: &str) -> ! {
        let server = simple_server::Server::new(move |request, response| {
            println!("Request received. {} {}", request.method(), request.uri());
            self.route(request, response)
        });

        server.listen(host, port);
    }

    fn route(&self, request: simple_server::Request<std::vec::Vec<u8>>, response: simple_server::Builder) -> simple_server::ResponseResult {
        match (request.method(), request.uri().path()) {
            (&Method::GET, "/hello") => {
                self.hello(response)
            }
            (_, _) => {
                self.four_oh_four(response)
            }
        }
    }
}

fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    let server = Server; 

    server.listen(host, port);
}
