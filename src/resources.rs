use iron::headers::ContentType;
use hyper::header::ContentType as HyperContent;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use std::fs::File;
use std::io::Read;

#[cfg(not(debug_assertions))] use handlers::StaticByteHandler;
#[cfg(debug_assertions)] use handlers::SingleFileHandler;
#[cfg(debug_assertions)] use std::path::Path;

#[cfg(not(debug_assertions))] static APP_CSS: &'static [u8] = include_bytes!("../resources/app.css");
#[cfg(not(debug_assertions))] static INDEX_HTML: &'static [u8] = include_bytes!("../resources/index.html");
#[cfg(not(debug_assertions))] static LOGIN_TEMPLATE: &'static str = include_string!("../resources/login.html");

fn css_content_type() -> HyperContent {
    HyperContent(Mime(TopLevel::Text, SubLevel::Css, vec![(Attr::Charset, Value::Utf8)]))
}

#[cfg(not(debug_assertions))]
pub fn create_index_handler() -> StaticByteHandler {
    StaticByteHandler::new(INDEX_HTML, ContentType::html())
}

#[cfg(not(debug_assertions))]
pub fn create_css_handler() -> StaticByteHandler {
    StaticByteHandler::new(APP_CSS, css_content_type())
}

#[cfg(debug_assertions)]
pub fn create_index_handler() -> SingleFileHandler {
    SingleFileHandler::new(Path::new("resources/index.html").to_owned(), ContentType::html())
}

#[cfg(debug_assertions)]
pub fn create_css_handler() -> SingleFileHandler {
    SingleFileHandler::new(Path::new("resources/app.css").to_owned(), css_content_type())
}

#[cfg(debug_assertions)]
pub fn get_login_template() -> String {
    let mut f = File::open("resources/login.html").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}

#[cfg(not(debug_assertions))]
pub fn get_login_template() -> String {
    String::from(LOGIN_TEMPLATE)
}
