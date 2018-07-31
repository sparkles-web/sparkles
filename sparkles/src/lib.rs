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

pub trait Response {
    fn to_response_result(self) -> ResponseResult;
}

impl Response for &'static str {
    fn to_response_result(self) -> ResponseResult {
        let mut response = Builder::new();

        Ok(response.body(self.as_bytes().to_vec())?)
    }
}

impl<E> Response for Result<simple_server::Response<Vec<u8>>, E> where E: std::fmt::Display {
    fn to_response_result(self) -> ResponseResult {
        let mut response = Builder::new();

        match self {
            Ok(o) => {
                Ok(o)
            },
            Err(e) => {
                Err(simple_server::Error::Other(e.to_string()))
            },
        }
    }
}

pub mod prelude {
    pub use crate::{Method, StatusCode, Builder, ResponseResult, Response};
    pub use sparkles_macros::*;
}