#[allow(clippy::module_inception)]
mod icon_button;
mod icon_button_color;
mod icon_button_high_contrast;
mod icon_button_loading;
mod icon_button_radius;
mod icon_button_size;
mod icon_button_variant;
mod icon_button_variant_ghost;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum IconButtonRoute {
    #[at("/")]
    Root,
    #[at("/color")]
    Color,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/loading")]
    Loading,
    #[at("/radius")]
    Radius,
    #[at("/size")]
    Size,
    #[at("/variant")]
    Variant,
    #[at("/variant-ghost")]
    VariantGhost,
}

pub fn render(route: IconButtonRoute) -> Html {
    match route {
        IconButtonRoute::Root => html! { <icon_button::IconButtonExample />},
        IconButtonRoute::Color => html! { <icon_button_color::IconButtonColorExample />},
        IconButtonRoute::HighContrast => {
            html! { <icon_button_high_contrast::IconButtonHighContrastExample />}
        }
        IconButtonRoute::Loading => html! { <icon_button_loading::IconButtonLoadingExample />},
        IconButtonRoute::Radius => html! { <icon_button_radius::IconButtonRadiusExample />},
        IconButtonRoute::Size => html! { <icon_button_size::IconButtonSizeExample />},
        IconButtonRoute::Variant => html! { <icon_button_variant::IconButtonVariantExample />},
        IconButtonRoute::VariantGhost => {
            html! { <icon_button_variant_ghost::IconButtonVariantGhostExample />}
        }
    }
}
