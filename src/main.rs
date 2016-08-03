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
use iron::headers::{ContentType, ContentDisposition, DispositionType, DispositionParam, Charset};
use router::Router;

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::borrow::Borrow;

use url::form_urlencoded;

static ICONS_128: &'static [u8] = include_bytes!("../resources/icons-128.png");
static ICONS_64: &'static [u8] = include_bytes!("../resources/icons-64.png");

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
    router.get("/img/icons-64.png", icons64);
    router.get("/img/icons-128.png", icons128);

    fn icons64(_: &mut Request) -> IronResult<Response> {
        let headers = Header(ContentType::png());
        Ok(Response::with((status::Ok, ICONS_64, headers)))
    }

    fn icons128(_: &mut Request) -> IronResult<Response> {
        let headers = Header(ContentType::png());
        Ok(Response::with((status::Ok, ICONS_128, headers)))
    }

    fn root_dir(_: &mut Request) -> IronResult<Response> {
        let headers = Header(ContentType::html());
        let rendered_page = rendering::files::render("Files", get_file_list(Path::new(".")).map(|x| x.unwrap()));

        Ok(Response::with((status::Ok, rendered_page, headers)))
    }

    fn download(req: &mut Request) -> IronResult<Response> {
        let query = req.url.query();

        let filepath = query.and_then(|q| {
            let params = form_urlencoded::parse(q.as_bytes());
            let param_map = param_map(params);

            let filenames = param_map.get("filename");
            filenames
                .and_then(|f| f.first())
                .and_then(|x| if x.is_empty() {None} else {
                    // FIXME: These shouldn't use unwrap and the serve_dir is an issue since it
                    // shouldn't need to happen everytime
                    let download_file = Path::new(x).canonicalize().unwrap();
                    let serve_dir = Path::new(".").canonicalize().unwrap();

                    if download_file.starts_with(serve_dir) && !download_file.is_dir() {
                        Some(download_file)
                    } else {
                        None
                    }
                })
        });

        if let Some(f) = filepath {
            let headers = Header(
                ContentDisposition {
                    disposition: DispositionType::Attachment,
                    parameters: vec![DispositionParam::Filename(
                        Charset::Us_Ascii,
                        None,
                        f.file_name().unwrap().to_str().unwrap().as_bytes().to_vec()
                    )]
                }
            );
            let resp = Response::with((status::Ok, f, headers));

            Ok(resp)
        } else {
            Ok(Response::with((status::Ok, "Not a valid file")))
        }
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
