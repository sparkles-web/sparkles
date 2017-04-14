use hyper;

pub struct Error {
    pub inner: hyper::Error,
}
