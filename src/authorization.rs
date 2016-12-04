use iron::middleware::BeforeMiddleware;
use iron::{Request, IronResult};
use iron::IronError;
use iron::modifiers::{RedirectRaw};
use std::io::{Error, ErrorKind};
use iron::status;
use iron_sessionstorage::SessionRequestExt;
use url::percent_encoding;

use iron::middleware::Chain;
use iron::Handler;
use iron_sessionstorage;

use time;

struct Login {
    login_time: time::Tm
}

impl iron_sessionstorage::Value for Login {
    fn get_key() -> &'static str {
        "login_time"
    }

    fn into_raw(self) -> String {
        time::strftime("%s", &self.login_time).unwrap()
    }

    fn from_raw(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            time::strptime(&value, "%s").ok().map(|x| Login {login_time: x})
        }
    }
}

pub struct AuthorizationMiddleware {
}

impl AuthorizationMiddleware {
    pub fn new() -> AuthorizationMiddleware {
        AuthorizationMiddleware {}
    }
}

pub fn secured_handler<H: Handler> (handler: H) -> Chain {
    let mut chain = Chain::new(handler);
    chain.link_before(AuthorizationMiddleware::new());
    chain
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