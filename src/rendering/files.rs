use std::fs::DirEntry;
use horrorshow::{RenderBox, Template};

fn render_file(entry: &DirEntry) -> Box<RenderBox> {
    let file_name = entry.file_name();
    box_html! {
        p : file_name.to_str().unwrap()
    }
}

pub fn render<I: Iterator<Item=DirEntry>>(title: &str, files: I) -> String {
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
                        @ for file in files {
                            : render_file(&file);
                        }
                    }
                }
            }
        }
    }).into_string().unwrap()
}
