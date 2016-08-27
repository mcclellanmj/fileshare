use iron::middleware::Handler;
use iron::{Request, Response, IronResult};
use iron::status;

use std::sync::Arc;
use iron::modifiers::Header;

use http::map_params;
use url::form_urlencoded;

use std::path::{Path, PathBuf};
use database::ShareDatabase;

use std::fs;
use http::headers::download_file_header;
use horrorshow::Template;
use std::fs::DirEntry;
use filetools::dir;
use iron::headers::ContentType;

use html;

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

fn render_shared_folder<I: Iterator<Item=DirEntry>>(files: I) -> String {
    let mut sorted_files = files.collect::<Vec<DirEntry>>();
    sorted_files.sort_by(dir::sort);
    let title = "Shared Folder";

    (html! {
        : raw!("<!DOCTYPE html>");
        html {
            : html::head(title);
            body {
                main {
                    header { h1 : title}
                    section(id="files") {
                        div(class="file-list") {
                        }
                    }
                }
            }
        }
    }).into_string().unwrap()
}

fn show_shared_folder(f: PathBuf) -> Response {
    let headers = Header(ContentType::html());

    let file_list = fs::read_dir(f).unwrap().map(|x| x.unwrap());
    let rendered_page = render_shared_folder(file_list);

    Response::with((status::Ok, rendered_page, headers))
}

fn download_response(f: PathBuf) -> Response {
    let download_header = Header(download_file_header(f.file_name().unwrap().to_str().unwrap()));
    Response::with((status::Ok, f, download_header))
}

impl Handler for AccessSharedHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let query = request.url.query();

        let file_path = query.and_then(|q| {
            let params = form_urlencoded::parse(q.as_bytes());
            let param_map = map_params(params);

            let hashes = param_map.get("hash");
            hashes
                .and_then(|f| f.first())
                .and_then(|x| if x.is_empty() {None} else {
                    let database = self.database.clone();
                    database.get_shared_by_hash(x).map(|x| Path::new(&x).to_owned())
                })
        });

        if let Some(f) = file_path {
            if f.is_dir() {
                Ok(show_shared_folder(f))
            } else {
                Ok(download_response(f))
            }
        } else {
            Ok(Response::with((status::BadRequest, "Invalid file")))
        }
    }
}