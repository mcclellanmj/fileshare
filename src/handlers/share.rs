use iron::{Request, Response, IronResult};
use iron::headers::ContentType;
use iron::status;
use iron::modifiers::Header;
use iron::middleware::Handler;

use std::io::Read;
use uuid::Uuid;
use std::sync::Arc;
use std::path::PathBuf;

use filetools;
use database::ShareDatabase;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct ShareRequest {
    full_path: PathBuf,
    email: String
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

        let filepath = if share_request.full_path.exists() && filetools::is_child_of(&serve_dir, &share_request.full_path) {
            Some(share_request.full_path.canonicalize().unwrap())
        } else {
            None
        };

        if let Some(f) = filepath {
            {
                let uuid = Uuid::new_v4().simple().to_string();
                let filepath = filetools::make_string(&f);

                let num_rows_added = self.connection.add_shared_file(&uuid, &String::from(filepath));
                println!("Shared file [{}] with uuid [{}] and added [{}] rows to database", filepath, uuid, num_rows_added);
            }

            let headers = Header(ContentType::json());
            Ok(Response::with((status::Ok, "Good", headers)))
        } else {
            Ok(Response::with((status::BadRequest, "No valid file found in the filename param, ensure that filename is set on url parameters and that it is a valid file")))
        }
    }
}