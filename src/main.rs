//FIXME: Check the router framework for updates to use Iron 0.4.0

#[macro_use]
extern crate horrorshow;
extern crate iron;
extern crate router;
extern crate url;

mod rendering;
mod filetools;

use iron::{Iron, Request, Response, IronResult};
use iron::modifiers::{Header};
use iron::status;
use iron::headers::ContentType;
use router::Router;

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::borrow::Borrow;

use url::form_urlencoded;

fn get_file_list(path: &Path) -> fs::ReadDir {
    fs::read_dir(path).unwrap()
}

fn param_map(params: form_urlencoded::Parse) -> HashMap<String, Vec<String>> {
    let mut map : HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in params {
        let borrowed_key: &str = key.borrow();
        if !map.contains_key(borrowed_key) {
            map.insert(borrowed_key.to_string(), Vec::new());
        } else {
            // Do nothing
        }

        map.get_mut(borrowed_key).unwrap().push(value.to_string());
    }

    return map;
}

fn main() {
    let mut router = Router::new();
    router.get("/", root_dir);
    router.get("/download", download);

    fn root_dir(_: &mut Request) -> IronResult<Response> {
        let headers = Header(ContentType::html());
        let rendered_page = rendering::files::render("Files", get_file_list(Path::new(".")).map(|x| x.unwrap()));

        Ok(Response::with((status::Ok, rendered_page, headers)))
    }

    fn download(req: &mut Request) -> IronResult<Response> {
        let query = req.url.clone().query;

        if let Some(q) = query {
            let params = form_urlencoded::parse(q.as_bytes());
            let param_map = param_map(params);

            let filenames = param_map.get("filename");
            let filename = filenames
                .and_then(|f| f.first())
                .and_then(|x| if x.is_empty() {None} else {
                    // FIXME: These shouldn't use unwrap and the serve_dir is an issue since it
                    // shouldn't need to happen everytime
                    let download_file = Path::new(x).canonicalize().unwrap();
                    let serve_dir = Path::new(".").canonicalize().unwrap();

                    if download_file.starts_with(serve_dir) {
                        Some(x)
                    } else {
                        None
                    }
                });

            if let Some(x) = filename {
                Ok(Response::with((format!("Sending you the file [{}]", x))))
            } else {
                Ok(Response::with(("Did not provide a valid file to download, sorry")))
            }
        } else {
            Ok(Response::with(("Did not provide a valid file to download, sorry")))
        }
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
