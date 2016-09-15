use iron::middleware::Handler;
use iron::{Request, Response, IronResult};
use iron::status;

use std::sync::Arc;
use iron::modifiers::Header;

use http::Params;

use std::path::{Path, PathBuf};
use database::ShareDatabase;

use std::fs;
use http::headers::download_file_header;
use std::fs::DirEntry;
use filetools::dir;
use iron::headers::ContentType;

pub struct AccessSharedHandler {
    database : Arc<ShareDatabase>
}

impl AccessSharedHandler {
    pub fn new(database: Arc<ShareDatabase>) -> AccessSharedHandler {
        AccessSharedHandler {
            database: database
        }
    }
}

fn show_shared_folder(f: PathBuf, hash: String) -> Response {
    let headers = Header(ContentType::html());

    let file_list = fs::read_dir(f).unwrap().map(|x| x.unwrap());

    Response::with((status::NotImplemented, "Not yet implemented", headers))
}

fn download_response(f: PathBuf) -> Response {
    let download_header = Header(download_file_header(f.file_name().unwrap().to_str().unwrap()));
    Response::with((status::Ok, f, download_header))
}

impl Handler for AccessSharedHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let url = request.url.clone().into_generic_url();
        let params = Params::new(&url);

        let hash_param = params.get_first_param(&String::from("hash"));
        let file_path = hash_param.and_then(|h| {
            if !h.is_empty() {
                let database: Arc<ShareDatabase> = self.database.clone();
                database.get_shared_by_hash(&h).map(|y| (String::from(h.clone()), Path::new(&y).to_owned()))
            } else {
                None
            }
        });

        if let Some((hash, f)) = file_path {
            if f.is_dir() {
                let requested_path = params.get_first_param(&String::from("filepath")).map(|x| Path::new(&x).to_owned());

                if let Some(path) = requested_path {
                    if !dir::is_child_of(&f, &path) {
                        Ok(Response::with((status::BadRequest, "Cannot access file outside of the shared folder")))
                    } else {
                        if path.is_dir() {
                            Ok(show_shared_folder(path, hash))
                        } else {
                            Ok(download_response(path))
                        }
                    }

                } else {
                    Ok(show_shared_folder(f, hash))
                }
            } else {
                Ok(download_response(f))
            }
        } else {
            Ok(Response::with((status::BadRequest, "Invalid file")))
        }
    }
}