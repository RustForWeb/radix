mod app;
mod r#box;
mod container;
mod decorative_box;
mod flex;
mod grid;
mod section;
mod switch;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    yew::Renderer::<App>::new().render();
}
