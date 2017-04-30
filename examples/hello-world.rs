extern crate sparkles;
extern crate futures;
extern crate serde_json;

use sparkles::Request;
use sparkles::Response;
use sparkles::Error;
use sparkles::ResponseBuilder;
use sparkles::Status;

use futures::BoxFuture;

use serde_json::value::Value;

fn main() {
    let addr = String::from("0.0.0.0:7878").parse().unwrap();
    let mut server = sparkles::Server::new("templates".to_string());

    server.add_route("/", root);

    server.run(&addr);
}

fn root(_: Request) -> BoxFuture<Response, Error> {
    let mut res = ResponseBuilder::new();
    res.with_template("hello-world".to_string());

    let name = Value::String(String::from("sparkles"));
    res.data.insert("name".to_string(), name);

    res.with_status(Status::Ok);

    res.to_response().into_future()
}
