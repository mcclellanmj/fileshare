use iron::middleware::Handler;
use iron::{Request, Response, IronResult};
use iron::headers::ContentType;
use iron::modifiers::{RedirectRaw, Header};
use url::form_urlencoded;
use uuid::Uuid;
use std::path::{Path, PathBuf};

use http::headers::download_file_header;
use http::map_params;

use database::ShareDatabase;
use std::sync::Arc;
use filetools;
use filetools::dir;
use iron::status;

mod access_shared;
mod filelist;
pub use self::access_shared::AccessSharedHandler;
pub use self::filelist::FilelistHandler;
pub use self::filelist::SharedFilelistHandler;

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

pub struct ShareHandler {
    root_folder: Arc<PathBuf>,
    connection: Arc<ShareDatabase>
}

impl ShareHandler {
    pub fn new(connection: Arc<ShareDatabase>, path: Arc<PathBuf>) -> ShareHandler {
        ShareHandler {
            connection: connection,
            root_folder: path
        }
    }
}

impl Handler for ShareHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let serve_dir = self.root_folder.clone();
        let query = req.url.query();

        let filepath = query.and_then(|q| {
            let params = form_urlencoded::parse(q.as_bytes());
            let param_map = map_params(params);

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

                let num_rows_added = self.connection.add_shared_file(&uuid, &String::from(filepath));
                println!("Shared file [{}] with uuid [{}] and added [{}] rows to database", filepath, uuid, num_rows_added);
            }

            let headers = Header(ContentType::html());

            Ok(Response::with((status::NotImplemented, "Not yet implemented", headers)))
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
            let param_map = map_params(params);

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
            let download_header = Header(download_file_header(f.file_name().unwrap().to_str().unwrap()));
            let resp = Response::with((status::Ok, f, download_header));
            Ok(resp)
        } else {
            Ok(Response::with((status::BadRequest, "No valid file found in the filename param, ensure that filename is set on url parameters and that it is a valid file")))
        }
    }
}