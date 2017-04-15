use iron;
use iron::middleware::Handler;
use iron::{Request, Response, IronResult};
use iron::headers::ContentType;
use iron::modifiers::{RedirectRaw, Header};
use url::form_urlencoded;
use std::path::{Path, PathBuf};

use http::headers::download_file_header;
use http::map_params;

use rustc_serialize::json;

use std::sync::Arc;
use iron::status;

use apierror;

mod filelist;
pub use self::filelist::FilelistHandler;
pub use self::filelist::SharedFilelistHandler;

mod share;
pub use self::share::ShareHandler;
pub use self::share::ShareDownloadHandler;

mod login;
pub use self::login::LoginFormHandler;
pub use self::login::AuthenticateHandler;

mod upload;
pub use self::upload::UploadHandler;

mod create_directory;
pub use self::create_directory::CreateDirectoryHandler;

#[allow(dead_code)]
pub struct StaticByteHandler {
    bytes: &'static [u8],
    content_type: ContentType
}

impl StaticByteHandler {
    #[allow(dead_code)]
    pub fn new(bytes: &'static [u8], content_type: ContentType) -> StaticByteHandler {
        StaticByteHandler {bytes: bytes, content_type: content_type}
    }
}

impl Handler for StaticByteHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let headers = Header(self.content_type.clone());
        Ok(Response::with((status::Ok, self.bytes, headers)))
    }
}

pub struct SingleFileHandler {
    path: PathBuf,
    content_type: ContentType
}

impl SingleFileHandler {
    pub fn new(path: PathBuf, content_type: ContentType) -> SingleFileHandler {
        SingleFileHandler {
            path: path,
            content_type: content_type
        }
    }
}

impl Handler for SingleFileHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let headers = Header(self.content_type.clone());
        Ok(Response::with((status::Ok, self.path.clone(), headers)))
    }
}

pub struct RedirectHandler {
    new_location: &'static str
}

impl RedirectHandler {
    pub fn new(new_location: &'static str) -> RedirectHandler {
        RedirectHandler {new_location: new_location}
    }
}

impl Handler for RedirectHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Found, RedirectRaw(String::from(self.new_location)))))
    }
}

pub struct DownloadHandler {
    root_folder: Arc<PathBuf>
}

impl DownloadHandler {
    pub fn new(path: Arc<PathBuf>) -> DownloadHandler {
        DownloadHandler {
            root_folder: path
        }
    }
}

impl Handler for DownloadHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let serve_dir = self.root_folder.clone();
        let query = apitry!(
            req.url.query().ok_or(
                apierror::ApiError::BadInput("Missing url query segment".to_string())),
                status::BadRequest
        );

        let filepath = {
            let params = form_urlencoded::parse(query.as_bytes());
            let param_map = map_params(params);

            let filenames = param_map.get("filename");
            filenames
                .and_then(|f| f.first())
                .and_then(|x| if x.is_empty() {None} else {
                    let download_file = Path::new(x).canonicalize().unwrap();

                    if download_file.starts_with(serve_dir.as_ref()) && !download_file.is_dir() {
                        Some(download_file)
                    } else {
                        None
                    }
                })
        };

        if let Some(f) = filepath {
            let download_header = Header(download_file_header(f.file_name().unwrap().to_str().unwrap()));
            let resp = Response::with((status::Ok, f, download_header));
            Ok(resp)
        } else {
            Ok(Response::with((status::BadRequest, "No valid file found in the filename param, ensure that filename is set on url parameters and that it is a valid file")))
        }
    }
}