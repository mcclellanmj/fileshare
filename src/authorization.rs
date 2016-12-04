use iron::middleware::BeforeMiddleware;
use iron::{Request, IronResult};
use iron::IronError;
use iron::modifiers::{RedirectRaw};
use std::io::{Error, ErrorKind};
use iron::status;
use iron_sessionstorage::SessionRequestExt;
use url::percent_encoding;
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
        let generic_url = req.url.clone().into_generic_url();
        let query = percent_encoding::utf8_percent_encode(
            generic_url.as_str(),
            percent_encoding::QUERY_ENCODE_SET);

        if !result {
            let response = (status::Found, RedirectRaw(String::from(format!("/login.html?url={}", query))));
            Err(IronError::new(Error::new(ErrorKind::Other, "Not Authenticated"), response))
        } else {
            Ok(())
        }
    }
}