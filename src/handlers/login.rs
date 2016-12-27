use liquid;
use liquid::{Renderable, LiquidOptions, Context, Value};

use iron::{Request, Response, IronResult};
use iron::middleware::Handler;
use iron::modifiers::Header;
use iron::headers::ContentType;
use iron::status;
use iron::prelude::Plugin;

use resources;
use http::Params;
use params::Params as IronParams;
use params::Value as ParamValue;

pub struct LoginFormHandler;

impl LoginFormHandler {
    pub fn new() -> LoginFormHandler {
        LoginFormHandler
    }
}

impl Handler for LoginFormHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let url = request.url.clone().into_generic_url();
        let params = Params::new(&url);

        let redirect_url = params.get_first_param(&"url".to_string()).unwrap_or("/".to_string());
        let template_string = resources::get_login_template();

        let template = liquid::parse(template_string.as_str(), LiquidOptions::default()).unwrap();

        let mut data = Context::new();
        data.set_val("success_url", Value::Str(redirect_url));

        let output = template.render(&mut data).unwrap().unwrap();

        Ok(Response::with((status::Ok, output, Header(ContentType::html()))))
    }
}

pub struct AuthenticateHandler {
    username: String,
    password: String
}

impl AuthenticateHandler {
    pub fn new<T: Into<String>>(username: T, password: T) -> AuthenticateHandler {
        AuthenticateHandler {username : username.into(), password : password.into()}
    }

    fn value_to_string(value: &ParamValue) -> Option<String> {
        match value {
            &ParamValue::String(ref v) => Some(v.clone()),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum AuthenticationStatus {
    Authenticated,
    InvalidCredentials,
    MissingFormParams
}

impl Handler for AuthenticateHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let params = req.get_ref::<IronParams>();
        println!("Password is {}, username is {}", self.username, self.password);

        let authenticated_status = if let Ok(map) = params {
            let ref btree = *map;
            let credentials = btree.get("username")
                .and_then(AuthenticateHandler::value_to_string)
                .and_then(|u| btree.get("password")
                    .and_then(AuthenticateHandler::value_to_string)
                    .map(|p| (u,p)));

            if let Some((username, password)) = credentials {
                if self.username == username && self.password == password {
                    AuthenticationStatus::Authenticated
                } else {
                    AuthenticationStatus::InvalidCredentials
                }
            } else {
                AuthenticationStatus::MissingFormParams
            }
        } else {
            AuthenticationStatus::MissingFormParams
        };

        match authenticated_status {
            AuthenticationStatus::Authenticated => Ok(Response::with(format!("{:?}", params))),
            _ => Ok(Response::with(format!("not authenticated, {:?}", authenticated_status)))
        }
    }
}
