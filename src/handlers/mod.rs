extern crate core;

use iron::middleware::Handler;
use iron::{Request, Response, IronResult};
use iron::headers::ContentType;
use iron::modifiers::{RedirectRaw, Header};
use rusqlite::Connection;
use url::form_urlencoded;
use uuid::Uuid;
use rendering::share;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use http;

use std::sync::{Mutex, Arc};
use filetools;
use filetools::dir;

use handlers::core::borrow::Borrow;

use iron::status;

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

pub struct StaticByteHandler {
   bytes: &'static [u8]
}

impl StaticByteHandler {
    pub fn new(bytes: &'static [u8]) -> StaticByteHandler {
        StaticByteHandler {bytes: bytes}
    }
}

impl Handler for StaticByteHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let headers = Header(ContentType::png());
        Ok(Response::with((status::Ok, self.bytes, headers)))
    }
}

pub struct RedirectHandler {
    new_location: &'static str
}

impl RedirectHandler {
    pub fn new(new_location: &'static str) -> RedirectHandler {
        RedirectHandler {new_location: new_location}
    }
}

impl Handler for RedirectHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Found, RedirectRaw(String::from(self.new_location)))))
    }
}

pub struct SharedHandler {}

impl SharedHandler {
    pub fn new() -> SharedHandler {
        SharedHandler {}
    }
}

impl Handler for SharedHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello Shared Thing")))
    }
}

pub struct ShareHandler {
    root_folder: Arc<PathBuf>,
    connection: Arc<Mutex<Connection>>
}

impl ShareHandler {
    pub fn new(connection: Arc<Mutex<Connection>>, path: Arc<PathBuf>) -> ShareHandler {
        ShareHandler {
            connection: connection,
            root_folder: path
        }
    }
}

impl Handler for ShareHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let serve_dir = self.root_folder.clone();
        let sqlite = self.connection.clone();
        let query = req.url.query();

        let filepath = query.and_then(|q| {
            let params = form_urlencoded::parse(q.as_bytes());
            let param_map = param_map(params);

            let filenames = param_map.get("filename");
            filenames
                .and_then(|f| f.first())
                .and_then(|x| if x.is_empty() {None} else {
                    let full_path = Path::new(x);

                    if full_path.exists() && dir::is_child_of(&serve_dir, &full_path.to_path_buf()) {
                        Some(full_path.canonicalize().unwrap())
                    } else {
                        None
                    }
                })
        });

        if let Some(f) = filepath {
            {
                let uuid = Uuid::new_v4().simple().to_string();
                let filepath = filetools::files::make_string(&f);

                let connection = sqlite.lock().unwrap();
                let num_rows_added = connection.execute_named("INSERT INTO shared_files(hash, path) VALUES (:hash, :path)", &[(":hash", &uuid), (":path", &filepath)]).unwrap();
                println!("Shared file [{}] with uuid [{}] and added [{}] rows to database", filepath, uuid, num_rows_added);
            }

            let headers = Header(ContentType::html());
            let rendered_page = share::render(f.as_path());

            Ok(Response::with((status::Ok, rendered_page, headers)))
        } else {
            Ok(Response::with((status::BadRequest, "No valid file found in the filename param, ensure that filename is set on url parameters and that it is a valid file")))
        }
    }
}

pub struct DownloadHandler {
    root_folder: Arc<PathBuf>
}

impl DownloadHandler {
    pub fn new(path: Arc<PathBuf>) -> DownloadHandler {
        DownloadHandler {
            root_folder: path
        }
    }
}

impl Handler for DownloadHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let serve_dir = self.root_folder.clone();
        let query = req.url.query();

        let filepath = query.and_then(|q| {
            let params = form_urlencoded::parse(q.as_bytes());
            let param_map = param_map(params);

            let filenames = param_map.get("filename");
            filenames
                .and_then(|f| f.first())
                .and_then(|x| if x.is_empty() {None} else {
                    let download_file = Path::new(x).canonicalize().unwrap();

                    if download_file.starts_with(serve_dir.as_ref()) && !download_file.is_dir() {
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
            Ok(Response::with((status::BadRequest, "No valid file found in the filename param, ensure that filename is set on url parameters and that it is a valid file")))
        }
    }
}