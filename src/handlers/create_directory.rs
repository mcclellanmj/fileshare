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

        println!("{:?}", create_directory_request);

        Ok(Response::with((status::NotImplemented)))
    }
}