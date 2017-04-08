use std::fmt;
use std::error;
use std::convert::From;
use rustc_serialize;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct ErrorPayload {
    message: String
}

impl ErrorPayload {
    pub fn create(from_err: &ApiError) -> ErrorPayload {
        ErrorPayload {
            message: format!("{}", from_err)
        }
    }
}

#[derive(Debug)]
pub enum ApiError {
    BadInput(String),
    AccessDenied(String)
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            ApiError::BadInput(ref message) => write!(f, "BadInput: {}", message),
            ApiError::AccessDenied(ref message) => write!(f, "AccessDenied: {}", message),
        }
    }
}

impl error::Error for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::BadInput(ref message) => message,
            ApiError::AccessDenied(ref message) => message
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ApiError::BadInput(_) => None,
            ApiError::AccessDenied(_) => None,
        }
    }
}

impl From<rustc_serialize::json::DecoderError> for ApiError {
    fn from(err: rustc_serialize::json::DecoderError) -> Self {
        ApiError::BadInput(format!("Could not parse JSON. Caused by {}", err))
    }
}

#[macro_export]
macro_rules! apitry {
    ($result:expr, $message:expr) => (apitry!($result, $message, iron::status::InternalServerError));
    ($result:expr, $message:expr, $status:expr) => {
        match $result {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(err) => {
                let from_err: apierror::ApiError = ::std::convert::From::from(err);
                let payload = json::encode(&apierror::ErrorPayload::create(&from_err)).unwrap();
                let headers = iron::modifiers::Header(iron::headers::ContentType::json());

                return ::std::result::Result::Err(iron::IronError::new(from_err, ($status, payload, headers)))
            }
        }
    };
}