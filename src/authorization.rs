use iron::middleware::BeforeMiddleware;
use iron::{Request, IronResult};
use iron::IronError;
use iron::modifiers::{RedirectRaw};
use std::io::{Error, ErrorKind};
use iron::status;
use iron_sessionstorage::SessionRequestExt;
use Login;

pub struct AuthorizationMiddleware {
    path_starts_with: String
}

impl AuthorizationMiddleware {
    pub fn new(path_starts_with: String) -> AuthorizationMiddleware {
        AuthorizationMiddleware { path_starts_with: path_starts_with }
    }
}

impl BeforeMiddleware for AuthorizationMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let result = try!(req.session().get::<Login>()).is_some();
        println!("{}", self.path_starts_with);
        if !result {
            let response = (status::Found, RedirectRaw(String::from("/login.html")));
            Err(IronError::new(Error::new(ErrorKind::Other, "Not Authenticated"), response))
        } else {
            Ok(())
        }
    }
}