#![feature(async_await, await_macro, futures_api, pin)]

#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use simple_server::{Method, StatusCode};
use futures::future;
use std::task::{self, Executor};
use std::sync::Arc;

/*
struct ResponseFuture;

impl std::future::Future for ResponseFuture {
    type Output = simple_server::ResponseResult;

    fn poll(self: std::mem::PinMut<'_, Self>, cx: &mut std::task::Context) -> std::task::Poll<Self::Output> {
        use std::task::Poll;

    }
}
*/

struct W;

impl task::Wake for W {
    fn wake(_: &Arc<Self>) {}
}

struct CurrentThread;

impl task::Executor for CurrentThread {
    fn spawn_obj(&mut self, mut task: std::future::FutureObj<'static, ()>) -> Result<(), task::SpawnObjError> {
        let waker = unsafe { task::local_waker(Arc::new(W)) };
        let mut ctx = task::Context::new(&waker, self);
        std::future::Future::poll(std::mem::PinMut::new(&mut task), &mut ctx);
        Ok(())
    }
}

struct Server;

impl Server {
    async fn hello(&self, mut response: simple_server::Builder) -> impl std::future::Future<Output=simple_server::ResponseResult> {
        future::ready(Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec()).unwrap()))
    }

    fn four_oh_four(&self, mut response: simple_server::Builder) -> simple_server::ResponseResult {
        response.status(StatusCode::NOT_FOUND);
        Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
    }

    fn listen(self, host: &str, port: &str) -> ! {
        let server = simple_server::Server::new(move |request, response| {
            println!("Request received. {} {}", request.method(), request.uri());
            Ok(CurrentThread.spawn_obj(Box::new(self.route(request, response)).into()))
        });

        server.listen(host, port);
    }

    async fn route(&self, request: simple_server::Request<std::vec::Vec<u8>>, response: simple_server::Builder) -> impl std::future::Future<Output=simple_server::ResponseResult> {
        match (request.method(), request.uri().path()) {
            (&Method::GET, "/hello") => {
                await!(self.hello(response))
            }
            (_, _) => {
                panic!("oh no")
                //self.four_oh_four(response)
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
