#[allow(clippy::module_inception)]
mod heading;
mod heading_align;
mod heading_as;
mod heading_color;
mod heading_high_contrast;
mod heading_size;
mod heading_trim;
mod heading_trim_box;
mod heading_truncate;
mod heading_weight;
mod heading_wrap;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum HeadingRoute {
    #[at("/")]
    Root,
    #[at("/align")]
    Align,
    #[at("/as")]
    As,
    #[at("/color")]
    Color,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/size")]
    Size,
    #[at("/trim")]
    Trim,
    #[at("/trim-box")]
    TrimBox,
    #[at("/truncate")]
    Truncate,
    #[at("/weight")]
    Weight,
    #[at("/wrap")]
    Wrap,
}

pub fn render(route: HeadingRoute) -> Html {
    match route {
        HeadingRoute::Root => html! { <heading::HeadingExample />},
        HeadingRoute::Align => html! { <heading_align::HeadingAlignExample />},
        HeadingRoute::As => html! { <heading_as::HeadingAsExample />},
        HeadingRoute::Color => html! { <heading_color::HeadingColorExample />},
        HeadingRoute::HighContrast => {
            html! { <heading_high_contrast::HeadingHighContrastExample />}
        }
        HeadingRoute::Size => html! { <heading_size::HeadingSizeExample />},
        HeadingRoute::Trim => html! { <heading_trim::HeadingTrimExample />},
        HeadingRoute::TrimBox => html! { <heading_trim_box::HeadingTrimBoxExample />},
        HeadingRoute::Truncate => html! { <heading_truncate::HeadingTruncateExample />},
        HeadingRoute::Weight => html! { <heading_weight::HeadingWeightExample />},
        HeadingRoute::Wrap => html! { <heading_wrap::HeadingWrapExample />},
    }
}
