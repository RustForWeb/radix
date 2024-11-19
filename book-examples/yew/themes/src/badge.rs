#[allow(clippy::module_inception)]
mod badge;
mod badge_color;
mod badge_high_contrast;
mod badge_radius;
mod badge_size;
mod badge_variant;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum BadgeRoute {
    #[at("/")]
    Root,
    #[at("/color")]
    Color,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/radius")]
    Radius,
    #[at("/size")]
    Size,
    #[at("/variant")]
    Variant,
}

pub fn render(route: BadgeRoute) -> Html {
    match route {
        BadgeRoute::Root => html! { <badge::BadgeExample />},
        BadgeRoute::Color => html! { <badge_color::BadgeColorExample />},
        BadgeRoute::HighContrast => html! { <badge_high_contrast::BadgeHighContrastExample />},
        BadgeRoute::Radius => html! { <badge_radius::BadgeRadiusExample />},
        BadgeRoute::Size => html! { <badge_size::BadgeSizeExample />},
        BadgeRoute::Variant => html! { <badge_variant::BadgeVariantExample />},
    }
}
