#[macro_use]
extern crate horrorshow;
extern crate iron;
extern crate router;
extern crate url;
extern crate persistent;

mod rendering;
mod filetools;

use iron::typemap::Key;
use iron::middleware::Chain;
use iron::{Iron, Request, Response, IronResult};
use iron::modifiers::{Header, RedirectRaw};
use iron::status;
use iron::headers::{ContentType, ContentDisposition, DispositionType, DispositionParam, Charset};
use router::Router;
use iron::Plugin;

use persistent::Read;

use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::borrow::Borrow;

use url::form_urlencoded;

static ICONS_128: &'static [u8] = include_bytes!("../resources/icons-128.png");
static ICONS_64: &'static [u8] = include_bytes!("../resources/icons-64.png");

#[derive(Clone, Copy)]
pub struct AppConfigKey;

pub struct AppConfig {
    root_folder: PathBuf,
}

impl Key for AppConfigKey { type Value = AppConfig; }

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
    router.get("/", direct_to_index);
    router.get("/index.html", index);
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

    fn direct_to_index(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Found, RedirectRaw(String::from("index.html")))))
    }

    fn index(_: &mut Request) -> IronResult<Response> {
        let headers = Header(ContentType::html());
        let rendered_page = rendering::files::render("Files", get_file_list(Path::new(".")).map(|x| x.unwrap()));

        Ok(Response::with((status::Ok, rendered_page, headers)))
    }

    fn download(req: &mut Request) -> IronResult<Response> {
        let serve_dir = req.get_ref::<Read<AppConfigKey>>().unwrap().root_folder.clone();
        let query = req.url.query();

        let filepath = query.and_then(|q| {
            let params = form_urlencoded::parse(q.as_bytes());
            let param_map = param_map(params);

            let filenames = param_map.get("filename");
            filenames
                .and_then(|f| f.first())
                .and_then(|x| if x.is_empty() {None} else {
                    let download_file = Path::new(x).canonicalize().unwrap();

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
    let app_config = AppConfig {
        root_folder : Path::new(".").canonicalize().unwrap()
    };

    let mut request_chain = Chain::new(router);
    request_chain.link_before(Read::<AppConfigKey>::one(app_config));

    Iron::new(request_chain).http("localhost:3000").unwrap();
}
