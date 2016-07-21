use std::cmp::Ordering;
use std::fs::DirEntry;

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
