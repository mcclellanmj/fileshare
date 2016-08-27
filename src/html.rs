use horrorshow::{RenderBox, RenderOnce};

const MAIN_CSS: &'static str = include_str!("../resources/main.css");

pub fn head<T: RenderOnce + 'static>(title: T) -> Box<RenderBox> {
    box_html! {
        head {
            style(type="text/css") : raw!(MAIN_CSS);
            title : title
        }
    }
}