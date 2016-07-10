extern crate iron;
extern crate time;

use iron::prelude::*;
use iron::BeforeMiddleware;
use std::io::Read;

pub struct RequestLogger;

impl BeforeMiddleware for RequestLogger {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        for x in req.url.path() {
            println!("[{}] - {} {} /{}", time::now().strftime("%Y-%m-%d %H:%M:%S %z").unwrap(), req.remote_addr, req.method, x);
        }

        let mut bytes = vec![];
        req.body.read_to_end(&mut bytes);
        let body = String::from_utf8(bytes).unwrap();
        println!("{}", body);

        println!("");

        Ok(())
    }
}
