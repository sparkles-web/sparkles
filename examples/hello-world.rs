extern crate sparkles;

use sparkles::App;

fn main() {
    let mut app = App::new();

    app.get("/", |_req, mut res| Ok(res.body("Hello Rust!".as_bytes())?));

    app.listen("127.0.0.1", "3000", || {
        println!("Example app listening on port 3000!");
    });
}
