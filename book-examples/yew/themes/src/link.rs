#[allow(clippy::module_inception)]
mod link;
mod link_color;
mod link_high_contrast;
mod link_size;
mod link_truncate;
mod link_underline;
mod link_weight;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum LinkRoute {
    #[at("/")]
    Root,
    #[at("/color")]
    Color,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/size")]
    Size,
    #[at("/truncate")]
    Truncate,
    #[at("/underline")]
    Underline,
    #[at("/weight")]
    Weight,
}

pub fn render(route: LinkRoute) -> Html {
    match route {
        LinkRoute::Root => html! { <link::LinkExample />},
        LinkRoute::Color => html! { <link_color::LinkColorExample />},
        LinkRoute::HighContrast => {
            html! { <link_high_contrast::LinkHighContrastExample />}
        }
        LinkRoute::Size => html! { <link_size::LinkSizeExample />},
        LinkRoute::Truncate => html! { <link_truncate::LinkTruncateExample />},
        LinkRoute::Underline => html! { <link_underline::LinkUnderlineExample />},
        LinkRoute::Weight => html! { <link_weight::LinkWeightExample />},
    }
}
