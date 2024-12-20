mod app;
#[cfg(feature = "aspect-ratio")]
mod aspect_ratio;
#[cfg(feature = "avatar")]
mod avatar;
#[cfg(feature = "badge")]
mod badge;
#[cfg(feature = "blockquote")]
mod blockquote;
#[cfg(feature = "box")]
mod r#box;
#[cfg(feature = "button")]
mod button;
#[cfg(feature = "callout")]
mod callout;
#[cfg(feature = "card")]
mod card;
#[cfg(feature = "checkbox")]
mod checkbox;
#[cfg(feature = "code")]
mod code;
#[cfg(feature = "container")]
mod container;
#[cfg(feature = "data-list")]
mod data_list;
#[cfg(any(
    feature = "box",
    feature = "container",
    feature = "flex",
    feature = "grid",
    feature = "section"
))]
mod decorative_box;
#[cfg(feature = "em")]
mod em;
#[cfg(feature = "flex")]
mod flex;
#[cfg(feature = "grid")]
mod grid;
#[cfg(feature = "heading")]
mod heading;
#[cfg(feature = "icon-button")]
mod icon_button;
#[cfg(feature = "inset")]
mod inset;
#[cfg(feature = "kbd")]
mod kbd;
#[cfg(feature = "link")]
mod link;
#[cfg(feature = "quote")]
mod quote;
#[cfg(feature = "radio")]
mod radio;
#[cfg(feature = "section")]
mod section;
#[cfg(feature = "select")]
mod select;
#[cfg(feature = "separator")]
mod separator;
#[cfg(feature = "skeleton")]
mod skeleton;
#[cfg(feature = "spinner")]
mod spinner;
#[cfg(feature = "strong")]
mod strong;
#[cfg(feature = "switch")]
mod switch;
#[cfg(feature = "table")]
mod table;
#[cfg(feature = "text")]
mod text;
#[cfg(feature = "text-area")]
mod text_area;
#[cfg(feature = "text-field")]
mod text_field;
#[cfg(feature = "tooltip")]
mod tooltip;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    yew::Renderer::<App>::new().render();
}
