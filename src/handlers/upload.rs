use std::sync::Arc;

use iron::{Request, Response, IronResult};
use iron::middleware::Handler;
use iron::modifiers::Header;
use iron::status;
use iron::prelude::Plugin;
use iron::headers::ContentType;

use std::path::{Path, PathBuf};

use params::Params as IronParams;
use params::Value as ParamValue;
use params::File as ParamFile;

use filetools;

use rustc_serialize::json;

#[derive(Debug)]
enum ParameterError {
    InvalidType,
    MissingParameter(String),
    ValidationError(String)
}

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

    // FIXME: Temporary, these functions need their own class
    fn extract_file_from_parameter(param: &ParamValue) -> Result<&ParamFile, ParameterError> {
        match param {
            &ParamValue::File(ref x) => Ok(x),
            _ => Err(ParameterError::InvalidType)
        }
    }

    fn extract_string_from_parameter(param: &ParamValue) -> Result<&String, ParameterError> {
        match param {
            &ParamValue::String(ref x) => Ok(x),
            _ => Err(ParameterError::InvalidType)
        }
    }
}

impl Handler for UploadHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let params = req.get_ref::<IronParams>().unwrap();

        let filepath = params
            .find(&["filepath"])
            .ok_or(ParameterError::MissingParameter("filepath".to_string()))
            .and_then(UploadHandler::extract_string_from_parameter)
            .and_then(|path_string| {
                let file_path = Path::new(path_string).to_owned();

                filetools::is_child_of_safe(&self.root_folder, &file_path)
                    .map_err(|_| ParameterError::ValidationError("Path is not valid".to_string()))
                    .map(|x| {
                        match x {
                            true => Ok(file_path),
                            false => Err(ParameterError::ValidationError("Path is not a child of server root".to_string()))
                        }
                    })
            });

        let param_file = filepath.and_then(|f| {
            let file_parameter = params.find(&["file"]);

            file_parameter.ok_or(ParameterError::MissingParameter("file".to_string()))
                .and_then(UploadHandler::extract_file_from_parameter)
        });

        println!("{:?}", param_file);


        // FIXME: Temporary until a later date
        let upload_response = UploadResponse { filepath: String::from("/bin") };
        let json = json::encode(&upload_response).unwrap();
        let headers = Header(ContentType::json());
        Ok(Response::with((status::Ok, json, headers)))
    }
}