extern crate iron;
extern crate request_logger;

use request_logger::RequestLogger;
use iron::prelude::*;
use iron::status;

fn main() {
    fn hello(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello")))
    }

    let mut chain = Chain::new(hello);
    chain.link_before(RequestLogger);
    Iron::new(chain).http("localhost:8000").unwrap();
    println!("Listening on port 8000...")
}
