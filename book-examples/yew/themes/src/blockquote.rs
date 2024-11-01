#[allow(clippy::module_inception)]
pub mod blockquote;
pub mod blockquote_color;
pub mod blockquote_high_contrast;
pub mod blockquote_size;
pub mod blockquote_truncate;
pub mod blockquote_weight;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum BlockquoteRoute {
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
    #[at("/weight")]
    Weight,
}

pub fn render(route: BlockquoteRoute) -> Html {
    match route {
        BlockquoteRoute::Root => html! { <blockquote::BlockquoteExample />},
        BlockquoteRoute::Color => html! { <blockquote_color::BlockquoteColorExample />},
        BlockquoteRoute::HighContrast => {
            html! { <blockquote_high_contrast::BlockquoteHighContrastExample />}
        }
        BlockquoteRoute::Size => html! { <blockquote_size::BlockquoteSizeExample />},
        BlockquoteRoute::Truncate => html! { <blockquote_truncate::BlockquoteTruncateExample />},
        BlockquoteRoute::Weight => html! { <blockquote_weight::BlockquoteWeightExample />},
    }
}
