use iron::{Request, Response, IronResult};
use iron::headers::ContentType;
use iron::status;
use iron::modifiers::Header;
use iron::middleware::Handler;

use std::io::Read;
use uuid::Uuid;
use std::sync::Arc;
use std::path::{Path,PathBuf};

use filetools;
use database::ShareDatabase;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct ShareRequest {
    full_path: String,
    email: String
}

#[derive(RustcDecodable, RustcEncodable)]
struct ShareResponse {
    uuid: String,
    url: String
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

        let mut request_body = String::new();
        req.body.read_to_string(&mut request_body).unwrap();
        let share_request: ShareRequest = json::decode(&request_body).unwrap();

        let filepath = {
            let path = Path::new(&share_request.full_path).to_owned();

            if path.exists() && filetools::is_child_of(&serve_dir, &path) {
                Some(path.canonicalize().unwrap())
            } else {
                None
            }
        };

        if let Some(f) = filepath {
            let uuid = Uuid::new_v4().simple().to_string();
            let filepath = filetools::make_string(&f);

            let num_rows_added = self.connection.add_shared_file(&uuid, &String::from(filepath));
            println!("Shared file [{}] with uuid [{}] and added [{}] rows to database", filepath, uuid, num_rows_added);

            let headers = Header(ContentType::json());

            let mut request_url = req.url.clone().into_generic_url();
            request_url.set_path("/shared/view");
            request_url.set_query(Some(&format!("hash={}", uuid)));

            let response = ShareResponse {
                uuid: uuid,
                url: request_url.into_string()
            };

            let response_json = json::encode(&response).unwrap();
            Ok(Response::with((status::Ok, response_json, headers)))
        } else {
            Ok(Response::with((status::BadRequest, "No valid file found in the filename param, ensure that filename is set on url parameters and that it is a valid file")))
        }
    }
}