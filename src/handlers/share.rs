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
use database::ShareResult::*;

use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;
use lettre::transport::sendmail::SendmailTransport;

use http::headers::download_file_header;
use http::Params;

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

    fn send_email(address: &str, link: String) {
        let email = EmailBuilder::new()
            .to((address, "Matt McClellan"))
            .from("mcclellan.mj@gmail.com")
            .subject("Rust Email")
            .text(link.as_str())
            .build()
            .unwrap();

        let mut sender = SendmailTransport::new();
        let result = sender.send(email);

        assert!(result.is_ok());
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

            let num_rows_added = self.connection.add_shared_file(&uuid, &filepath.to_string());
            println!("Shared file [{}] with uuid [{}] and added [{}] rows to database", filepath, uuid, num_rows_added);

            let headers = Header(ContentType::json());

            let mut request_url = req.url.clone().into_generic_url();
            request_url.set_path("/shared/download");
            request_url.set_query(Some(&format!("hash={}", uuid)));

            ShareHandler::send_email("mcclellan.mj@gmail.com", request_url.clone().into_string());

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

pub struct ShareDownloadHandler {
    database: Arc<ShareDatabase>
}

impl ShareDownloadHandler {
    pub fn new(connection: Arc<ShareDatabase>) -> ShareDownloadHandler {
        ShareDownloadHandler {
            database: connection
        }
    }
}
impl Handler for ShareDownloadHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let url = request.url.clone().into_generic_url();
        let params = Params::new(&url);

        if let Some(hash) = params.get_first_param(&"hash".to_string()) {
            let database = self.database.clone();

            match database.get_shared_by_hash(&hash) {
                Missing => Ok(Response::with((status::BadRequest, "No shared file with this hash"))),
                Expired => Ok(Response::with((status::BadRequest, "Share has expired"))),
                Valid(ref x) => {
                    let shared_path = Path::new(x).to_owned();
                    let download_header = Header(download_file_header(shared_path.file_name().unwrap().to_str().unwrap()));
                    Ok(Response::with((status::Ok, shared_path, download_header)))
                }
            }
        } else {
            Ok(Response::with((status::BadRequest, "Missing the hash query parameter")))
        }
    }
}
