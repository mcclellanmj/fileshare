use iron::headers::ContentType;

#[cfg(not(debug_assertions))] use handlers::StaticByteHandler;
#[cfg(debug_assertions)] use handlers::SingleFileHandler;
#[cfg(debug_assertions)] use std::path::Path;

#[cfg(not(debug_assertions))] static APP_CSS: &'static [u8] = include_bytes!("../resources/app.css");
#[cfg(not(debug_assertions))] static INDEX_HTML: &'static [u8] = include_bytes!("../resources/index.html");

#[cfg(not(debug_assertions))]
pub fn create_index_handler() -> StaticByteHandler {
    StaticByteHandler::new(INDEX_HTML, ContentType::html())
}

#[cfg(not(debug_assertions))]
pub fn create_css_handler() -> StaticByteHandler {
    StaticByteHandler::new(APP_CSS, ContentType::plaintext())
}

#[cfg(debug_assertions)]
pub fn create_index_handler() -> SingleFileHandler {
    SingleFileHandler::new(Path::new("resources/index.html").to_owned(), ContentType::html())
}

#[cfg(debug_assertions)]
pub fn create_css_handler() -> SingleFileHandler {
    SingleFileHandler::new(Path::new("resources/app.css").to_owned(), ContentType::plaintext())
}