#![deny(warnings)]
#[macro_use]
extern crate iron;
extern crate iron_sessionstorage;
extern crate router;
extern crate url;
extern crate rusqlite;
extern crate uuid;
extern crate rustc_serialize;
extern crate hyper;
extern crate lettre;
extern crate time;

mod filetools;
mod http;
mod handlers;
mod database;
mod resources;

use iron::middleware::Chain;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::Router;
use handlers::{RedirectHandler, ShareHandler, DownloadHandler, FilelistHandler, SharedFilelistHandler, ShareDownloadHandler};

use std::path::Path;
use std::sync::Arc;

// use iron_sessionstorage::traits::*;
// use iron_sessionstorage::SessionStorage;
// use iron_sessionstorage::backends::SignedCookieBackend;

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

fn main() {
    let mut router = Router::new();    
    let database = Arc::new(database::ShareDatabase::new("data.sql"));
    let root_folder = Arc::new(Path::new(".").canonicalize().unwrap());

    router.get("/js/elm.js", frontend);
    router.get("/", RedirectHandler::new("index.html"));
    router.get("/index.html", resources::create_index_handler());
    router.get("/css/app.css", resources::create_css_handler());
    router.get("/view", FilelistHandler::new(root_folder.clone()));
    router.get("/shared/view", SharedFilelistHandler::new(database.clone()));
    router.get("/shared/download", ShareDownloadHandler::new(database.clone()));
    router.get("/download", DownloadHandler::new(root_folder.clone()));
    router.post("/share", ShareHandler::new(database.clone(), root_folder.clone()));

    fn frontend(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, Path::new("elm.js"))))
    }

    let request_chain = Chain::new(router);
    Iron::new(request_chain).http("localhost:3000").unwrap();
}
