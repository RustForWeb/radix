#[allow(clippy::module_inception)]
mod text_area;
mod text_area_color;
mod text_area_radius;
mod text_area_resize;
mod text_area_size;
mod text_area_variant;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum TextAreaRoute {
    #[at("/")]
    Root,
    #[at("/color")]
    Color,
    #[at("/radius")]
    Radius,
    #[at("/resize")]
    Resize,
    #[at("/size")]
    Size,
    #[at("/variant")]
    Variant,
}

pub fn render(route: TextAreaRoute) -> Html {
    match route {
        TextAreaRoute::Root => html! { <text_area::TextAreaExample /> },
        TextAreaRoute::Color => html! { <text_area_color::TextAreaColorExample /> },
        TextAreaRoute::Radius => html! { <text_area_radius::TextAreaRadiusExample /> },
        TextAreaRoute::Resize => html! { <text_area_resize::TextAreaResizeExample /> },
        TextAreaRoute::Size => html! { <text_area_size::TextAreaSizeExample /> },
        TextAreaRoute::Variant => html! { <text_area_variant::TextAreaVariantExample /> },
    }
}
