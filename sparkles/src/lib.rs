#![feature(
    async_await,
    await_macro,
    futures_api,
    pin,
    use_extern_macros
)]
#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

#[allow(rust_2018_idioms)] // https://github.com/rust-lang/rust/issues/52140
pub extern crate simple_server;
#[allow(rust_2018_idioms)] // https://github.com/rust-lang/rust/issues/52140
pub extern crate futures;

pub use simple_server::{Method, StatusCode, Builder, ResponseResult};

pub mod prelude {
    pub use crate::{Method, StatusCode, Builder, ResponseResult};
    pub use sparkles_macros::*;
}