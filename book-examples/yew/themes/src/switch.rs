#[allow(clippy::module_inception)]
mod switch;
mod switch_alignment;
mod switch_color;
mod switch_disabled;
mod switch_high_contrast;
mod switch_radius;
mod switch_size;
mod switch_variant;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum SwitchRoute {
    #[at("/")]
    Root,
    #[at("/alignment")]
    Alignment,
    #[at("/color")]
    Color,
    #[at("/disabled")]
    Disabled,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/radius")]
    Radius,
    #[at("/size")]
    Size,
    #[at("/variant")]
    Variant,
}

pub fn render(route: SwitchRoute) -> Html {
    match route {
        SwitchRoute::Root => html! { <switch::SwitchExample /> },
        SwitchRoute::Alignment => html! { <switch_alignment::SwitchAlignmentExample /> },
        SwitchRoute::Color => html! { <switch_color::SwitchColorExample /> },
        SwitchRoute::Disabled => html! { <switch_disabled::SwitchDisabledExample /> },
        SwitchRoute::HighContrast => html! { <switch_high_contrast::SwitchHighContrastExample /> },
        SwitchRoute::Radius => html! { <switch_radius::SwitchRadiusExample /> },
        SwitchRoute::Size => html! { <switch_size::SwitchSizeExample />},
        SwitchRoute::Variant => html! { <switch_variant::SwitchVariantExample />},
    }
}
