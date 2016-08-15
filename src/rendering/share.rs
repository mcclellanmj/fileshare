use horrorshow::Template;
use std::path::Path;

const MAIN_CSS: &'static str = include_str!("../../resources/main.css");

pub fn render(file: &Path) -> String {
    let title = if file.is_dir() {
        "Share Folder"
    } else {
        "Share File"
    };

    (html! {
        : raw!("<!DOCTYPE html>");
        html {
            head {
                style(type="text/css") : raw!(MAIN_CSS);
                title : title
            }
            body {
                main {
                    header { h1 : title}
                    div : format!("Going to share {}", file.file_name().unwrap().to_str().unwrap())
                }
            }
        }
    }).into_string().unwrap()
}
