#[allow(clippy::module_inception)]
mod separator;
mod separator_color;
mod separator_orientation;
mod separator_size_horizontal;
mod separator_size_vertical;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum SeparatorRoute {
    #[at("/")]
    Root,
    #[at("/color")]
    Color,
    #[at("/orientation")]
    Orientation,
    #[at("/size-horizontal")]
    SizeHorizontal,
    #[at("/size-vertical")]
    SizeVertical,
}

pub fn render(route: SeparatorRoute) -> Html {
    match route {
        SeparatorRoute::Root => html! { <separator::SeparatorExample/> },
        SeparatorRoute::Color => html! { <separator_color::SeparatorColorExample /> },
        SeparatorRoute::Orientation => {
            html! { <separator_orientation::SeparatorOrientationExample /> }
        }
        SeparatorRoute::SizeHorizontal => {
            html! { <separator_size_horizontal::SeparatorSizeHorizontalExample /> }
        }
        SeparatorRoute::SizeVertical => {
            html! { <separator_size_vertical::SeparatorSizeVerticalExample /> }
        }
    }
}
