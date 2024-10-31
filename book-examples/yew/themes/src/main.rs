mod app;
mod avatar;
mod blockquote;
mod r#box;
mod button;
mod code;
mod container;
mod decorative_box;
mod em;
mod flex;
mod grid;
mod heading;
mod kbd;
mod link;
mod quote;
mod section;
mod select;
mod strong;
mod switch;
mod text;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    yew::Renderer::<App>::new().render();
}
