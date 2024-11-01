#[allow(clippy::module_inception)]
mod text_field;
mod text_field_color;
mod text_field_radius;
mod text_field_size;
mod text_field_size_buttons;
mod text_field_variant;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum TextFieldRoute {
    #[at("/")]
    Root,
    #[at("/color")]
    Color,
    #[at("/radius")]
    Radius,
    #[at("/size")]
    Size,
    #[at("/size-buttons")]
    SizeButtons,
    #[at("/variant")]
    Variant,
}

pub fn render(route: TextFieldRoute) -> Html {
    match route {
        TextFieldRoute::Root => html! { <text_field::TextFieldExample /> },
        TextFieldRoute::Color => html! { <text_field_color::TextFieldColorExample /> },
        TextFieldRoute::Radius => html! { <text_field_radius::TextFieldRadiusExample /> },
        TextFieldRoute::Size => html! { <text_field_size::TextFieldSizeExample /> },
        TextFieldRoute::SizeButtons => {
            html! { <text_field_size_buttons::TextFieldSizeButtonsExample /> }
        }
        TextFieldRoute::Variant => html! { <text_field_variant::TextFieldVariantExample /> },
    }
}
