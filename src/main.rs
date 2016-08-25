#[macro_use]
extern crate horrorshow;
extern crate iron;
extern crate router;
extern crate url;
extern crate rusqlite;
extern crate uuid;

mod rendering;
mod filetools;
mod http;
mod handlers;

use iron::middleware::Chain;
use iron::{Iron, Request, Response, IronResult};
use iron::modifiers::Header;
use iron::status;
use iron::headers::ContentType;
use router::Router;
use iron::Plugin;
use handlers::{StaticByteHandler, RedirectHandler, SharedHandler, ShareHandler, DownloadHandler};

use rendering::files;

use std::fs;
use std::path::Path;

use rusqlite::Connection;
use std::sync::Mutex;
use std::sync::Arc;

static ICONS_128: &'static [u8] = include_bytes!("../resources/icons-128.png");
static ICONS_64: &'static [u8] = include_bytes!("../resources/icons-64.png");
static ICONS_32: &'static [u8] = include_bytes!("../resources/icons-32.png");

fn get_file_list(path: &Path) -> fs::ReadDir {
    fs::read_dir(path).unwrap()
}

fn main() {
    let mut router = Router::new();    
    let connection = Connection::open("data.sql").unwrap();
    {
        let mut stmt = connection.prepare("SELECT name FROM sqlite_master WHERE type='table' and name='shared_files'").unwrap();
        let table_exists = stmt.exists(&[]).unwrap();

        if !table_exists {
            connection.execute("CREATE TABLE shared_files(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                hash CHAR(36) UNIQUE,
                path VARCHAR(32768))", &[]).unwrap();

            connection.execute("CREATE UNIQUE INDEX shared_files_hash_index on shared_files (hash)", &[]).unwrap();
        }
    }

    let root_folder = Arc::new(Path::new(".").canonicalize().unwrap());
    let sqlite = Arc::new(Mutex::new(connection));

    router.get("/", RedirectHandler::new("index.html"));
    router.get("/index.html", index);
    router.get("/shared", SharedHandler::new());
    router.get("/download", DownloadHandler::new(root_folder.clone()));
    router.get("/share", ShareHandler::new(sqlite.clone(), root_folder.clone()));
    router.get("/img/icons-32.png", StaticByteHandler::new(ICONS_32));
    router.get("/img/icons-64.png", StaticByteHandler::new(ICONS_64));
    router.get("/img/icons-128.png", StaticByteHandler::new(ICONS_128));

    fn index(_: &mut Request) -> IronResult<Response> {
        let headers = Header(ContentType::html());
        let rendered_page = files::render("Files", get_file_list(Path::new(".")).map(|x| x.unwrap()));

        Ok(Response::with((status::Ok, rendered_page, headers)))
    }

    let request_chain = Chain::new(router);
    Iron::new(request_chain).http("localhost:3000").unwrap();
}
