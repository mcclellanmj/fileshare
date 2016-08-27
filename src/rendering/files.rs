use std::fs::DirEntry;
use horrorshow::{RenderBox, Template};
use filetools::dir;
use html;

fn render_file (entry: &DirEntry) -> Box<RenderBox> {
    let file_name = entry.file_name();
    let file_type = entry.file_type().unwrap();
    let full_path = String::from(entry.path().into_os_string().to_str().unwrap());

    let offset = if file_type.is_dir() {"icon-folder"} else {"icon-file"};
    box_html! {
        div(class="file-entry") {
            a(class="file-link", href="#") {
                span(class=format!("entry-icon {}", offset)) : raw!("");
                span : file_name.to_str().unwrap();
            }
            a(href=format!("/share?filename={}", full_path)) : raw!("Share");
        }
    }
}

pub fn render<I: Iterator<Item=DirEntry>>(title: &str, files: I) -> String {
    let mut sorted_files = files.collect::<Vec<DirEntry>>();
    sorted_files.sort_by(dir::sort);

    (html! {
        : raw!("<!DOCTYPE html>");
        html {
            : html::head(String::from(title));
            body {
                main {
                    header { h1 : title}
                    section(id="files") {
                        div(class="file-list") {
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
