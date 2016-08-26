use iron::middleware::Handler;
use iron::{Request, Response, IronResult};
use iron::status;

use rusqlite::Connection;
use std::sync::{Mutex, Arc};

use http::map_params;
use url::form_urlencoded;

use std::path::Path;

use filetools;

pub struct AccessSharedHandler {
    connection : Arc<Mutex<Connection>>
}

impl AccessSharedHandler {
    pub fn new(connection: Arc<Mutex<Connection>>) -> AccessSharedHandler {
        AccessSharedHandler {
            connection: connection
        }
    }
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
                    let connection = self.connection.lock().unwrap();
                    let mut stmt = connection.prepare("SELECT path FROM shared_files WHERE hash=:hash").unwrap();
                    let mut rows = stmt.query_named(&[(":hash", x)]).unwrap();

                    if let Some(r) = rows.next() {
                        let result = r.unwrap();
                        let path: String = result.get(0);
                        Some(Path::new(&path).to_owned())
                    } else {
                        None
                    }
                })
        });

        if let Some(f) = file_path {
            Ok(Response::with((status::Ok, filetools::files::make_string(&f))))
        } else {
            Ok(Response::with((status::BadRequest, "Invalid file")))
        }
    }
}