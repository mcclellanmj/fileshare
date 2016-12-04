use iron::middleware::BeforeMiddleware;
use iron::{Request, IronResult};
use iron::IronError;
use iron::modifiers::{RedirectRaw};
use std::io::{Error, ErrorKind};
use iron::status;
// use iron::request::Url;
use iron_sessionstorage::SessionRequestExt;
use Login;

pub struct AuthorizationMiddleware {
}

impl AuthorizationMiddleware {
    pub fn new() -> AuthorizationMiddleware {
        AuthorizationMiddleware {}
    }
}

impl BeforeMiddleware for AuthorizationMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let result = try!(req.session().get::<Login>()).is_some();

        if !result {
            let response = (status::Found, RedirectRaw(String::from("/login.html")));
            Err(IronError::new(Error::new(ErrorKind::Other, "Not Authenticated"), response))
        } else {
            Ok(())
        }
    }
}