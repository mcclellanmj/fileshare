use horrorshow::Template;
use std::path::Path;
use html;

pub fn render(file: &Path) -> String {
    let title = if file.is_dir() {
        "Share Folder"
    } else {
        "Share File"
    };

    (html! {
        : raw!("<!DOCTYPE html>");
        html {
            : html::head(title);
            body {
                main {
                    header { h1 : title}
                    div : format!("Going to share {}", file.file_name().unwrap().to_str().unwrap())
                }
            }
        }
    }).into_string().unwrap()
}
