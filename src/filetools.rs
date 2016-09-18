use std::path::PathBuf;

pub fn make_string<'a> (path : &'a PathBuf) -> &'a str {
    path.to_str().unwrap()
}

pub fn is_child_of(parent: &PathBuf, child: &PathBuf) -> bool {
    let child_file = child.canonicalize().unwrap();
    let parent_file = parent.canonicalize().unwrap();

    child_file.starts_with(parent_file)
}