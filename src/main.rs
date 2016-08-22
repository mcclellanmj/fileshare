#[macro_use]
extern crate horrorshow;
extern crate iron;
extern crate router;
extern crate url;
extern crate persistent;
extern crate rusqlite;
extern crate uuid;

mod rendering;
mod filetools;
mod http;

use iron::typemap::Key;
use iron::middleware::Chain;
use iron::{Iron, Request, Response, IronResult};
use iron::modifiers::Header;
use iron::status;
use iron::headers::ContentType;
use router::Router;
use iron::Plugin;
use http::handlers::{StaticByteHandler, RedirectHandler};

use rendering::{files,share};
use persistent::Read;

use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::borrow::Borrow;

use url::form_urlencoded;
use rusqlite::Connection;
use std::sync::Mutex;
use std::sync::Arc;

use uuid::Uuid;

static ICONS_128: &'static [u8] = include_bytes!("../resources/icons-128.png");
static ICONS_64: &'static [u8] = include_bytes!("../resources/icons-64.png");
static ICONS_32: &'static [u8] = include_bytes!("../resources/icons-32.png");

#[derive(Clone, Copy)]
pub struct AppStateKey;

pub struct AppState {
    root_folder: PathBuf,
    sqlite: Arc<Mutex<Connection>>
}

impl Key for AppStateKey { type Value = AppState; }

fn get_file_list(path: &Path) -> fs::ReadDir {
    fs::read_dir(path).unwrap()
}

fn param_map(params: form_urlencoded::Parse) -> HashMap<String, Vec<String>> {
    let mut map : HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in params {
        let borrowed_key: &str = key.borrow();
        if !map.contains_key(borrowed_key) {
            map.insert(borrowed_key.to_string(), Vec::new());
        } else {
            // Do nothing
        }

        map.get_mut(borrowed_key).unwrap().push(value.to_string());
    }

    return map;
}

fn within_root(root: &PathBuf, target: &PathBuf) -> bool {
    let target_file = target.canonicalize().unwrap();

    target_file.starts_with(root)
}

fn share(req: &mut Request) -> IronResult<Response> {
    let serve_dir = req.get_ref::<Read<AppStateKey>>().unwrap().root_folder.clone();
    let sqlite = req.get_ref::<Read<AppStateKey>>().unwrap().sqlite.clone();
    let query = req.url.query();

    let filepath = query.and_then(|q| {
        let params = form_urlencoded::parse(q.as_bytes());
        let param_map = param_map(params);

        let filenames = param_map.get("filename");
        filenames
            .and_then(|f| f.first())
            .and_then(|x| if x.is_empty() {None} else {
                let full_path = Path::new(x);

                if full_path.exists() && within_root(&serve_dir, &full_path.to_path_buf()) {
                    Some(full_path.canonicalize().unwrap())
                } else {
                    None
                }
            })
    });

    if let Some(f) = filepath {
        let connection = sqlite.lock().unwrap();
        let uuid = Uuid::new_v4().simple().to_string();
        // TODO: Use uuid and path
        let result = connection.execute_named("INSERT INTO shared_files(hash, path) VALUES (:hash, :path)", &[(":hash", &uuid), (":path", &"")]).unwrap();

        let headers = Header(ContentType::html());
        let rendered_page = share::render(f.as_path());

        Ok(Response::with((status::Ok, rendered_page, headers)))
    } else {
        Ok(Response::with((status::Ok, "No valid file found in the filename param")))
    }
}

fn main() {
    let mut router = Router::new();

    router.get("/", RedirectHandler::new("index.html"));
    router.get("/index.html", index);
    router.get("/download", download);
    router.get("/share", share);
    router.get("/img/icons-32.png", StaticByteHandler::new(ICONS_32));
    router.get("/img/icons-64.png", StaticByteHandler::new(ICONS_64));
    router.get("/img/icons-128.png", StaticByteHandler::new(ICONS_128));

    fn index(_: &mut Request) -> IronResult<Response> {
        let headers = Header(ContentType::html());
        let rendered_page = files::render("Files", get_file_list(Path::new(".")).map(|x| x.unwrap()));

        Ok(Response::with((status::Ok, rendered_page, headers)))
    }

    fn download(req: &mut Request) -> IronResult<Response> {
        let serve_dir = req.get_ref::<Read<AppStateKey>>().unwrap().root_folder.clone();
        let query = req.url.query();

        let filepath = query.and_then(|q| {
            let params = form_urlencoded::parse(q.as_bytes());
            let param_map = param_map(params);

            let filenames = param_map.get("filename");
            filenames
                .and_then(|f| f.first())
                .and_then(|x| if x.is_empty() {None} else {
                    let download_file = Path::new(x).canonicalize().unwrap();

                    if download_file.starts_with(serve_dir) && !download_file.is_dir() {
                        Some(download_file)
                    } else {
                        None
                    }
                })
        });

        if let Some(f) = filepath {
            let download_header = Header(http::headers::download_file_header(f.file_name().unwrap().to_str().unwrap()));
            let resp = Response::with((status::Ok, f, download_header));
            Ok(resp)
        } else {
            Ok(Response::with((status::Ok, "Not a valid file")))
        }
    }

    // FIXME: Shouldn't just unwrap
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

    let app_config = AppState {
        root_folder : Path::new(".").canonicalize().unwrap(),
        sqlite : Arc::new(Mutex::new(connection))
    };

    let mut request_chain = Chain::new(router);
    request_chain.link_before(Read::<AppStateKey>::one(app_config));

    Iron::new(request_chain).http("localhost:3000").unwrap();
}
