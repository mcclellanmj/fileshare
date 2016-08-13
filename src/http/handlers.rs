use iron::middleware::Handler;
use iron::{Request, Response, IronResult};
use iron::headers::ContentType;
use iron::modifiers::Header;

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
