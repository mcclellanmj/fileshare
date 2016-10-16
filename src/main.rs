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
mod resources;

use iron::middleware::Chain;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::Router;
use handlers::{RedirectHandler, ShareHandler, DownloadHandler, FilelistHandler, SharedFilelistHandler};

use std::path::Path;
use std::sync::Arc;

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
    router.get("/download", DownloadHandler::new(root_folder.clone()));
    router.post("/share", ShareHandler::new(database.clone(), root_folder.clone()));

    fn frontend(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, Path::new("elm.js"))))
    }

    let request_chain = Chain::new(router);
    Iron::new(request_chain).http("localhost:3000").unwrap();
}
