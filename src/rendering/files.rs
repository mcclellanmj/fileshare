use std::fs::DirEntry;
use horrorshow::{RenderBox, Template};
use filetools::dir;

fn render_file(entry: &DirEntry) -> Box<RenderBox> {
    let file_name = entry.file_name();
    box_html! {
        p : file_name.to_str().unwrap()
    }
}

pub fn render<I: Iterator<Item=DirEntry>>(title: &str, files: I) -> String {
    let mut sorted_files = files.collect::<Vec<DirEntry>>();
    sorted_files.sort_by(dir::sort);

    (html! {
        : raw!("<!DOCTYPE html>");
        html {
            head {
                title : title
            }
            body {
                main {
                    header { h1 : title}
                    section(id="files") {
                        @ for file in sorted_files {
                            : render_file(&file);
                        }
                    }
                }
            }
        }
    }).into_string().unwrap()
}
