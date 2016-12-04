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
mod authorization;

use iron::middleware::Chain;
use iron::Iron;
use router::Router;
use handlers::{RedirectHandler, ShareHandler, SingleFileHandler, DownloadHandler, FilelistHandler, SharedFilelistHandler, ShareDownloadHandler};
use hyper::header::ContentType as HyperContent;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use authorization::secured_handler;

use std::path::Path;
use std::sync::Arc;

// use iron_sessionstorage::traits::*;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

fn js_content_type() -> HyperContent {
    HyperContent(Mime(TopLevel::Text, SubLevel::Javascript, vec![(Attr::Charset, Value::Utf8)]))
}

fn main() {
    let mut router = Router::new();    
    let database = Arc::new(database::ShareDatabase::new("data.sql"));
    let root_folder = Arc::new(Path::new(".").canonicalize().unwrap());

    // Unsecured resources
    router.get("/js/elm.js", SingleFileHandler::new(Path::new("elm.js").to_owned(), js_content_type()));
    router.get("/", RedirectHandler::new("index.html"));
    router.get("/css/app.css", resources::create_css_handler());
    router.get("/shared/view", SharedFilelistHandler::new(database.clone()));
    router.get("/shared/download", ShareDownloadHandler::new(database.clone()));
    router.get("/login.html", resources::create_login_handler());

    // Secured resources
    router.get("/index.html", secured_handler(resources::create_index_handler()));
    router.get("/view", secured_handler(FilelistHandler::new(root_folder.clone())));
    router.get("/download", secured_handler(DownloadHandler::new(root_folder.clone())));
    router.post("/share", secured_handler(ShareHandler::new(database.clone(), root_folder.clone())));

    let mut request_chain = Chain::new(router);
    request_chain.link_around(SessionStorage::new(SignedCookieBackend::new(b"NotASecret".to_vec())));

    Iron::new(request_chain).http("localhost:3000").unwrap();
}
