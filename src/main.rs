#![feature(async_await, await_macro, futures_api, pin)]

#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use simple_server::{Method, StatusCode};

struct Server;

async fn hello(server: &Server, mut response: simple_server::Builder) -> simple_server::ResponseResult {
    Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec()).unwrap())
}

async fn four_oh_four(server: &Server, mut response: simple_server::Builder) -> simple_server::ResponseResult {
    response.status(StatusCode::NOT_FOUND);
    Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
}

impl Server {
    fn listen(self, host: &str, port: &str) -> ! {
        let server = simple_server::Server::new(move |request, response| {
            println!("Request received. {} {}", request.method(), request.uri());

            futures::executor::block_on(route(&self, request, response))
        });

        server.listen(host, port);
    }
}

async fn route(server: &Server, request: simple_server::Request<std::vec::Vec<u8>>, response: simple_server::Builder) -> simple_server::ResponseResult {
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/hello") => {
            await!(hello(server, response))
        }
        (_, _) => {
            await!(four_oh_four(server, response))
        }
    }
}

fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    let server = Server; 

    server.listen(host, port);
}