use std::sync::Arc;

use iron::{Request, Response, IronResult};
use iron::middleware::Handler;
use iron::headers::ContentType;
use iron::status;
use iron::modifiers::Header;

use std::path::{Path, PathBuf};
use std::fs;
use std::fs::DirEntry;
use std::cmp::Ordering;
use std::cmp::Ord;

use filetools;
use http::Params;

use database::ShareDatabase;
use database::ShareResult::*;

use rustc_serialize::{json, Encodable, Encoder};

struct FileDetails {
    short_name: String,
    full_path: String,
    is_folder: bool,
    size: u64
}

impl Encodable for FileDetails {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("FileDetails", 1, |x| {
            try!(x.emit_struct_field("shortName", 0, |s| self.short_name.encode(s)));
            try!(x.emit_struct_field("fullPath", 1, |s| self.full_path.encode(s)));
            try!(x.emit_struct_field("isFolder", 2, |s| self.is_folder.encode(s)));
            try!(x.emit_struct_field("size", 3, |s| self.size.encode(s)));

            Ok(())
        })
    }
}

impl FileDetails {
    fn from_dir_entry(entry: &DirEntry) -> FileDetails {
        let metadata = entry.metadata().unwrap();

        FileDetails {
            full_path: String::from(filetools::make_string(&entry.path())),
            short_name: entry.file_name().into_string().unwrap(),
            size: metadata.len(),
            is_folder: metadata.is_dir()
        }
    }
}

fn sorting(x: &FileDetails, y: &FileDetails) -> Ordering {
    match (x.is_folder, y.is_folder) {
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        _ => x.short_name.cmp(&y.short_name)
    }
}

fn get_file_list(path: &Path) -> fs::ReadDir {
    fs::read_dir(path).unwrap()
}

fn json_folder_listing(path: &Path) -> String {
    let files = get_file_list(path);
    let mut file_infos = files.map(|x| FileDetails::from_dir_entry(&x.unwrap())).collect::<Vec<FileDetails>>();
    file_infos.sort_by(sorting);

    json::encode(&file_infos).unwrap()
}

pub struct FilelistHandler {
    root_folder: Arc<PathBuf>,
}

impl FilelistHandler {
    pub fn new(root: Arc<PathBuf>) -> FilelistHandler {
        FilelistHandler{root_folder: root}
    }
}

impl Handler for FilelistHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let url = request.url.clone().into_generic_url();
        let params = Params::new(&url);
        let root = self.root_folder.clone();

        let folder_to_view = params
            .get_first_param(&String::from("folder_path"))
            .map(|ref x| Path::new(x).to_owned())
            .unwrap_or(root.as_ref().clone());

        if folder_to_view.is_dir() {
            let json = json_folder_listing(&folder_to_view);
            let headers = Header(ContentType::json());
            Ok(Response::with((status::Ok, json, headers)))
        } else {
            Ok(Response::with((status::BadRequest, "Requested path is not a folder.")))
        }
    }
}

pub struct SharedFilelistHandler {
    database: Arc<ShareDatabase>
}

impl SharedFilelistHandler {
    pub fn new(database: Arc<ShareDatabase>) -> SharedFilelistHandler {
        SharedFilelistHandler {
            database: database
        }
    }
}

impl Handler for SharedFilelistHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let url = request.url.clone().into_generic_url();
        let params = Params::new(&url);

        // Get the shared parameter and fetch it from the database
        if let Some(hash) = params.get_first_param(&"hash".to_string()) {
            let database = self.database.clone();

            match database.get_shared_by_hash(&hash) {
                Missing => Ok(Response::with((status::BadRequest, "Invalid hash"))),
                Expired => Ok(Response::with((status::BadRequest, "Share has expired"))),
                Valid(ref x) => {
                    let shared_path = Path::new(x).to_owned();
                    let path = params.get_first_param(&"folder_path".to_string())
                        .map(|y| Path::new(&y).to_owned())
                        .or(Some(shared_path.clone()));

                    let folder = path.unwrap();

                    if filetools::is_child_of(&shared_path, &folder) {
                        if folder.is_dir() {
                            let headers = Header(ContentType::json());
                            Ok(Response::with((status::Ok, json_folder_listing(&folder), headers)))
                        } else {
                            Ok(Response::with((status::BadRequest, "Shared resource is not a directory")))
                        }
                    } else {
                        Ok(Response::with((status::BadRequest, "Cannot access files outside of shared directory")))
                    }
                }
            }
        } else {
            Ok(Response::with((status::BadRequest, "Invalid or Missing the hash")))
        }
    }
}
