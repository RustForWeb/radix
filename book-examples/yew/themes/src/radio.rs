#[allow(clippy::module_inception)]
mod radio;
mod radio_alignment;
mod radio_color;
mod radio_disabled;
mod radio_high_contrast;
mod radio_size;
mod radio_variant;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum RadioRoute {
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
    #[at("/size")]
    Size,
    #[at("/variant")]
    Variant,
}

pub fn render(route: RadioRoute) -> Html {
    match route {
        RadioRoute::Root => html! { <radio::RadioExample />},
        RadioRoute::Alignment => {
            html! { <radio_alignment::RadioAlignmentExample />}
        }
        RadioRoute::Color => html! { <radio_color::RadioColorExample />},
        RadioRoute::Disabled => html! { <radio_disabled::RadioDisabledExample />},
        RadioRoute::HighContrast => {
            html! { <radio_high_contrast::RadioHighContrastExample />}
        }
        RadioRoute::Size => html! { <radio_size::RadioSizeExample />},
        RadioRoute::Variant => html! { <radio_variant::RadioVariantExample />},
    }
}
