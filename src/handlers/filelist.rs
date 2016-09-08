use std::sync::Arc;

use iron::{Request, Response, IronResult};
use iron::middleware::Handler;
use iron::headers::ContentType;
use iron::status;
use iron::modifiers::Header;

use std::path::{Path, PathBuf};
use std::fs;
use std::fs::DirEntry;
use filetools::files;

use http::Params;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct FileDetails {
    short_name: String,
    full_path: String,
    is_folder: bool,
    size: u64
}

impl FileDetails {
    fn from_dir_entry(entry: &DirEntry) -> FileDetails {
        let metadata = entry.metadata().unwrap();

        FileDetails {
            full_path: String::from(files::make_string(&entry.path())),
            short_name: entry.file_name().into_string().unwrap(),
            size: metadata.len(),
            is_folder: metadata.is_dir()
        }
    }
}

fn get_file_list(path: &Path) -> fs::ReadDir {
    fs::read_dir(path).unwrap()
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
            .map(|ref x| Arc::new(Path::new(x).to_owned()))
            .unwrap_or(root);

        let files = get_file_list(folder_to_view.as_ref());
        let file_infos = files.map(|x| FileDetails::from_dir_entry(&x.unwrap())).collect::<Vec<FileDetails>>();

        let json = json::encode(&file_infos).unwrap();

        let headers = Header(ContentType::json());
        Ok(Response::with((status::Ok, json, headers)))
    }
}