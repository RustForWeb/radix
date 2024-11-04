mod app;

#[cfg(feature = "aspect-ratio")]
mod aspect_ratio;
#[cfg(feature = "avatar")]
mod avatar;
#[cfg(feature = "label")]
mod label;
#[cfg(feature = "select")]
mod select;
#[cfg(feature = "separator")]
mod separator;
#[cfg(feature = "switch")]
mod switch;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    yew::Renderer::<App>::new().render();
}
