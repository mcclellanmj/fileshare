use std::path::PathBuf;
use std::io::Error;

pub fn make_string<'a> (path : &'a PathBuf) -> &'a str {
    path.to_str().unwrap()
}

pub fn is_child_of_safe(parent: &PathBuf, child: &PathBuf) -> Result<bool, Error> {
    child.canonicalize()
        .and_then(|x| {
            parent.canonicalize().map(|y| (y, x))
        })
        .map(|(parent, child)| {
            child.starts_with(parent)
        })
}

pub fn is_child_of(parent: &PathBuf, child: &PathBuf) -> bool {
    let child_file = child.canonicalize().unwrap();
    let parent_file = parent.canonicalize().unwrap();

    child_file.starts_with(parent_file)
}