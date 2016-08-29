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
use horrorshow::{Template, RenderBox};
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

fn render_file (entry: &DirEntry, hash: String) -> Box<RenderBox> {
    let file_name = entry.file_name();
    let file_type = entry.file_type().unwrap();
    let full_path = String::from(entry.path().into_os_string().to_str().unwrap());

    let offset = if file_type.is_dir() {"icon-folder"} else {"icon-file"};
    box_html! {
        div(class="file-entry") {
            a(class="file-link", href=format!("/shared?hash={}&filepath={}", hash, full_path)) {
                span(class=format!("entry-icon {}", offset)) : raw!("");
                span : file_name.to_str().unwrap();
            }
        }
    }
}

fn render_shared_folder<I: Iterator<Item=DirEntry>>(files: I, hash: String) -> String {
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
                            @ for file in sorted_files {
                                : render_file(&file, hash.clone());
                            }
                        }
                    }
                }
            }
        }
    }).into_string().unwrap()
}

fn show_shared_folder(f: PathBuf, hash: String) -> Response {
    let headers = Header(ContentType::html());

    let file_list = fs::read_dir(f).unwrap().map(|x| x.unwrap());
    let rendered_page = render_shared_folder(file_list, hash);

    Response::with((status::Ok, rendered_page, headers))
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
                let folder_to_show = params
                    .get_first_param(&String::from("filepath"))
                    .map(|x| Path::new(&x).to_owned())
                    .and_then(|x| {
                        if dir::is_child_of(&f, &x) {
                            Some(x)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(f);
                Ok(show_shared_folder(folder_to_show, hash))
            } else {
                Ok(download_response(f))
            }
        } else {
            Ok(Response::with((status::BadRequest, "Invalid file")))
        }
    }
}