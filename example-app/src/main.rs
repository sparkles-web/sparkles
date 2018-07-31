#![feature(
    async_await,
    await_macro,
    futures_api,
    pin,
    use_extern_macros
)]
#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use sparkles::prelude::*;

#[server]
struct Server;

impl Server {
    #[route(GET, "/hello")]
    fn hello(&self, response: Builder) -> impl Response {
        "<h1>Hello Rust</h1>"
    }

    #[route(GET, "/error")]
    fn oh_no(&self, response: Builder) -> impl Response {
        Err("<h1>500</h1><p>Oh no!</p>")
    }
}

#[serve]
fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    let server = Server;

    server.listen(host, port);
}