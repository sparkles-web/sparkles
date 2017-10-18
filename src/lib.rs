extern crate simple_server;

pub use simple_server::{Request, Response, ResponseBuilder, Error};

pub type Handler = fn(Request<&[u8]>, ResponseBuilder) -> Result<Response<&[u8]>, Error>;

pub struct App {
    server: Option<simple_server::Server>,
}


impl App {
    pub fn new() -> App {
        App { server: None }
    }

    pub fn get(&mut self, _path: &str, handler: Handler) {
        self.server = Some(simple_server::Server::new(handler));
    }

    pub fn listen(self, host: &str, port: &str, _hook: fn()) {
        _hook(); // eventually pass this through when simple-server gains it
        self.server.map(|server| { server.listen(host, port); });
    }
}
