#[allow(clippy::module_inception)]
pub mod avatar;
pub mod avatar_color;
pub mod avatar_fallback;
pub mod avatar_high_contrast;
pub mod avatar_radius;
pub mod avatar_size;
pub mod avatar_variant;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum AvatarRoute {
    #[at("/")]
    Root,
    #[at("/color")]
    Color,
    #[at("/fallback")]
    Fallback,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/radius")]
    Radius,
    #[at("/size")]
    Size,
    #[at("/variant")]
    Variant,
}

pub fn render(route: AvatarRoute) -> Html {
    match route {
        AvatarRoute::Root => html! { <avatar::AvatarExample />},
        AvatarRoute::Color => html! { <avatar_color::AvatarColorExample />},
        AvatarRoute::Fallback => html! { <avatar_fallback::AvatarFallbackExample />},
        AvatarRoute::HighContrast => html! { <avatar_high_contrast::AvatarHighContrastExample />},
        AvatarRoute::Radius => html! { <avatar_radius::AvatarRadiusExample />},
        AvatarRoute::Size => html! { <avatar_size::AvatarSizeExample />},
        AvatarRoute::Variant => html! { <avatar_variant::AvatarVariantExample />},
    }
}
