use std::fs::DirEntry;
use horrorshow::{RenderBox, Template};
use filetools::dir;

const MAIN_CSS: &'static str = include_str!("../../resources/main.css");

fn render_file(entry: &DirEntry) -> Box<RenderBox> {
    let file_name = entry.file_name();
    box_html! {
        li {
            span(class="file-list-img") : raw!("");
            a(href="#") : file_name.to_str().unwrap()
        }
    }
}

pub fn render<I: Iterator<Item=DirEntry>>(title: &str, files: I) -> String {
    let mut sorted_files = files.collect::<Vec<DirEntry>>();
    sorted_files.sort_by(dir::sort);

    (html! {
        : raw!("<!DOCTYPE html>");
        html {
            head {
                style(type="text/css") : MAIN_CSS;
                title : title
            }
            body {
                main {
                    header { h1 : title}
                    section(id="files") {
                        ul(class="file-list") {
                            @ for file in sorted_files {
                                : render_file(&file);
                            }
                        }
                    }
                }
            }
        }
    }).into_string().unwrap()
}
