use std::path::PathBuf;

pub fn make_string<'a> (path : &'a PathBuf) -> &'a str {
	path.to_str().unwrap()
}