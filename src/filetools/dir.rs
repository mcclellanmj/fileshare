use std::cmp::Ordering;
use std::fs::DirEntry;
use std::path::PathBuf;

pub fn sort(x: &DirEntry, y: &DirEntry) -> Ordering {
    let meta_x = x.metadata().unwrap();
    let meta_y = y.metadata().unwrap();

    if meta_x.is_dir() && !meta_y.is_dir() {
        return Ordering::Less
    } else if !meta_x.is_dir() && meta_y.is_dir() {
        return Ordering::Greater
    } else {
        return x.file_name().cmp(&y.file_name())
    }
}

pub fn is_child_of(parent: &PathBuf, child: &PathBuf) -> bool {
    let child_file = child.canonicalize().unwrap();
    let parent_file = parent.canonicalize().unwrap();

    child_file.starts_with(parent_file)
}