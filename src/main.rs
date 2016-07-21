#[macro_use]
extern crate horrorshow;
extern crate iron;

mod rendering;
mod filetools;

use iron::{Iron, Request, Response, IronResult};
use iron::modifiers::{Header};
use iron::status;
use iron::headers::ContentType;

use std::fs;
use std::path::Path;

fn get_file_list(path: &Path) -> fs::ReadDir {
    fs::read_dir(path).unwrap()
}

fn main() {
    fn root_dir(_: &mut Request) -> IronResult<Response> {
        let headers = Header(ContentType::html());
        let rendered_page = rendering::files::render("Files", get_file_list(Path::new(".")).map(|x| x.unwrap()));

        Ok(Response::with((status::Ok, rendered_page, headers)))
    }

    Iron::new(root_dir).http("localhost:3000").unwrap();
}
