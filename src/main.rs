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
use iron::modifiers::Header;
use iron::status;
use iron::headers::ContentType;
use router::Router;
use handlers::{StaticByteHandler, RedirectHandler, AccessSharedHandler, ShareHandler, DownloadHandler, FilelistHandler, SharedFilelistHandler};

use std::fs;
use std::path::Path;

use std::sync::Arc;

static ICONS_128: &'static [u8] = include_bytes!("../resources/icons-128.png");
static ICONS_64: &'static [u8] = include_bytes!("../resources/icons-64.png");
static ICONS_32: &'static [u8] = include_bytes!("../resources/icons-32.png");

fn get_file_list(path: &Path) -> fs::ReadDir {
    fs::read_dir(path).unwrap()
}

fn main() {
    let mut router = Router::new();    
    let database = Arc::new(database::ShareDatabase::new("data.sql"));
    let root_folder = Arc::new(Path::new(".").canonicalize().unwrap());

    router.get("/frontend.html", frontend);
    router.get("/index.html", RedirectHandler::new("frontend.html"));
    router.get("/", RedirectHandler::new("frontend.html"));
    router.get("/view", FilelistHandler::new(root_folder.clone()));
    router.get("/shared/view", SharedFilelistHandler::new(database.clone()));
    router.get("/shared", AccessSharedHandler::new(database.clone()));
    router.get("/download", DownloadHandler::new(root_folder.clone()));
    router.get("/share", ShareHandler::new(database.clone(), root_folder.clone()));
    router.get("/img/icons-32.png", StaticByteHandler::new(ICONS_32));
    router.get("/img/icons-64.png", StaticByteHandler::new(ICONS_64));
    router.get("/img/icons-128.png", StaticByteHandler::new(ICONS_128));

    fn frontend(_: &mut Request) -> IronResult<Response> {
        let headers = Header(ContentType::html());
        Ok(Response::with((status::Ok, Path::new("frontend.html"), headers)))
    }

    let request_chain = Chain::new(router);
    Iron::new(request_chain).http("localhost:3000").unwrap();
}
