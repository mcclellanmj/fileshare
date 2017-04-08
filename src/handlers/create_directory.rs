use std::sync::Arc;

use iron;
use iron::{Request, Response, IronResult};
use iron::middleware::Handler;
use iron::status;

use std::path::{PathBuf};
use std::fs::create_dir;
use std::io::Read;

use apierror;

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
        apitry!(req.body.read_to_string(&mut request_body), 
            "Unable to read request body",
            status::BadRequest);

        let create_directory_request: CreateDirectoryRequest = apitry!(
            json::decode(&request_body),
            "Could not parse json",
            status::BadRequest
        );

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