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
    fn hello(&self, mut response: Builder) -> ResponseResult {
        Ok(response.body("<h1>Hello Rust!</h1>".as_bytes().to_vec())?)
    }

    #[not_found]
    fn four_oh_four(&self, mut response: Builder) -> ResponseResult {
        response.status(StatusCode::NOT_FOUND);
        Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
    }
}

#[serve]
fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    let server = Server;
}