use iron::middleware::Handler;
use iron::{Request, Response, IronResult};
use iron::headers::ContentType;
use iron::modifiers::{RedirectRaw, Header};

use iron::status;

pub struct StaticByteHandler {
   bytes: &'static [u8]
}

impl StaticByteHandler {
    pub fn new(bytes: &'static [u8]) -> StaticByteHandler {
        StaticByteHandler {bytes: bytes}
    }
}

impl Handler for StaticByteHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let headers = Header(ContentType::png());
        Ok(Response::with((status::Ok, self.bytes, headers)))
    }
}

pub struct RedirectHandler {
    new_location: &'static str
}

impl RedirectHandler {
    pub fn new(new_location: &'static str) -> RedirectHandler {
        RedirectHandler {new_location: new_location}
    }
}

impl Handler for RedirectHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Found, RedirectRaw(String::from(self.new_location)))))
    }
}
