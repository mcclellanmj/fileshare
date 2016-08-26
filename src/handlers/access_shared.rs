use iron::middleware::Handler;
use iron::{Request, Response, IronResult};
use iron::status;

use std::sync::Arc;
use iron::modifiers::Header;

use http::map_params;
use url::form_urlencoded;

use std::path::PathBuf;
use database::ShareDatabase;

use http::headers::download_file_header;

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

fn download_response(f: PathBuf) -> Response {
    let download_header = Header(download_file_header(f.file_name().unwrap().to_str().unwrap()));
    Response::with((status::Ok, f, download_header))
}

impl Handler for AccessSharedHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let query = request.url.query();

        let file_path = query.and_then(|q| {
            let params = form_urlencoded::parse(q.as_bytes());
            let param_map = map_params(params);

            let hashes = param_map.get("hash");
            hashes
                .and_then(|f| f.first())
                .and_then(|x| if x.is_empty() {None} else {
                    let database = self.database.clone();

                    database.get_shared_by_hash(x)
                })
        });

        if let Some(f) = file_path {
            Ok(download_response(f))
        } else {
            Ok(Response::with((status::BadRequest, "Invalid file")))
        }
    }
}