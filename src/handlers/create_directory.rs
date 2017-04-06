use std::sync::Arc;

use iron::IronError;
use iron::{Request, Response, IronResult};
use iron::middleware::Handler;
use iron::modifiers::Header;
use iron::status;
use iron::prelude::Plugin;
use iron::headers::ContentType;

use std::path::{Path, PathBuf};
use std::fs::copy;
use std::fs::create_dir;
use std::io::Read;

use params::Params as IronParams;
use params::Value as ParamValue;
use params::File as ParamFile;

use filetools;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct CreateDirectoryRequest {
    base_path: String,
    new_directory: String
}

pub struct CreateDirectoryHandler {
    root_folder: Arc<PathBuf>,
}

impl CreateDirectoryHandler {
    pub fn new(root: Arc<PathBuf>) -> CreateDirectoryHandler {
        CreateDirectoryHandler{root_folder: root}
    }
}

impl Handler for CreateDirectoryHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut request_body = String::new();
        req.body.read_to_string(&mut request_body).unwrap();
        let create_directory_request: CreateDirectoryRequest = itry!(json::decode(&request_body));

        let mut request_path = PathBuf::from(create_directory_request.base_path);
        request_path.push(create_directory_request.new_directory);

        let is_child = itry!(filetools::is_child_of_safe(&self.root_folder, &request_path.parent().unwrap().to_path_buf()));

        if !is_child {
            Ok(Response::with((status::BadRequest, "Trying to create directory outside of root")))
        } else {
            if request_path.as_path().exists() {
                Ok(Response::with((status::BadRequest, "Folder already exists")))
            } else {
                itry!(create_dir(request_path));
                Ok(Response::with((status::Ok, "null")))
            }
        }
    }
}