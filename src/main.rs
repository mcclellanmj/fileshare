#[macro_use]
extern crate iron;
extern crate router;
extern crate url;
extern crate rusqlite;
extern crate uuid;
extern crate rustc_serialize;

mod filetools;
mod http;
mod handlers;
mod database;

use iron::middleware::Chain;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use iron::headers::ContentType;
use router::Router;
use handlers::{StaticByteHandler, RedirectHandler, ShareHandler, DownloadHandler, FilelistHandler, SharedFilelistHandler};

use std::path::Path;

use std::sync::Arc;

static ICONS_128: &'static [u8] = include_bytes!("../resources/icons-128.png");
static ICONS_64: &'static [u8] = include_bytes!("../resources/icons-64.png");
static ICONS_MOBILE: &'static [u8] = include_bytes!("../resources/icons-mobile.png");
static INDEX_HTML: &'static [u8] = include_bytes!("../resources/index.html");

fn main() {
    let mut router = Router::new();    
    let database = Arc::new(database::ShareDatabase::new("data.sql"));
    let root_folder = Arc::new(Path::new(".").canonicalize().unwrap());

    router.get("/js/elm.js", frontend);
    router.get("/", RedirectHandler::new("index.html"));
    router.get("/index.html", StaticByteHandler::new(INDEX_HTML, ContentType::html()));
    router.get("/view", FilelistHandler::new(root_folder.clone()));
    router.get("/shared/view", SharedFilelistHandler::new(database.clone()));
    router.get("/download", DownloadHandler::new(root_folder.clone()));
    router.post("/share", ShareHandler::new(database.clone(), root_folder.clone()));
    router.get("/img/icons-mobile.png", StaticByteHandler::new(ICONS_MOBILE, ContentType::png()));
    router.get("/img/icons-64.png", StaticByteHandler::new(ICONS_64, ContentType::png()));
    router.get("/img/icons-128.png", StaticByteHandler::new(ICONS_128, ContentType::png()));

    fn frontend(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, Path::new("elm.js"))))
    }

    let request_chain = Chain::new(router);
    Iron::new(request_chain).http("localhost:3000").unwrap();
}
