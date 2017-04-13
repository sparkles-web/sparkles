use futures;
use futures::future::Future;
use futures::BoxFuture;

use error::Error;
use status::Status;

use std;
// Rename type for crate
type BTreeMap = std::collections::BTreeMap<String, Value>;

use serde_json::value::Value;

pub struct Response {
    pub data: BTreeMap,
    pub template: String,
    pub status: Status,
}

impl Response {
    /// just a tiny bit of ergonomics
    pub fn into_future(self) -> BoxFuture<Response, Error> {
        futures::future::ok(self).boxed()
    }
}
