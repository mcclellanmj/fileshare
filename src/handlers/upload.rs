use std::sync::Arc;

use iron::{Request, Response, IronResult};
use iron::middleware::Handler;
use iron::modifiers::Header;
use iron::status;
use iron::prelude::Plugin;
use iron::headers::ContentType;

use std::path::PathBuf;

use params::Params as IronParams;
use params::Value as ParamValue;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct UploadResponse {
    filepath: String
}

pub struct UploadHandler {
    root_folder: Arc<PathBuf>,
}

impl UploadHandler {
    pub fn new(root: Arc<PathBuf>) -> UploadHandler {
        UploadHandler{root_folder: root}
    }
}

impl Handler for UploadHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let params = req.get_ref::<IronParams>();

        // FIXME: Temporary until a later date
        let upload_response = UploadResponse { filepath: String::from("/bin") };
        let json = json::encode(&upload_response).unwrap();
        let headers = Header(ContentType::json());
        Ok(Response::with((status::Ok, json, headers)))
    }
}