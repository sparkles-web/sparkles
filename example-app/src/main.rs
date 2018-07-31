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
    fn hello(&self, mut response: Builder) -> impl Response {
        "<h1>Hello Rust</h1>"
    }
}

#[serve]
fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    let server = Server;
}