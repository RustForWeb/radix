#[allow(clippy::module_inception)]
mod button;
mod button_color;
mod button_high_contrast;
mod button_loading;
mod button_loading_spinner;
mod button_radius;
mod button_size;
mod button_variant;
mod button_variant_ghost;
mod button_with_icons;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum ButtonRoute {
    #[at("/")]
    Root,
    #[at("/color")]
    Color,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/loading")]
    Loading,
    #[at("/loading-spinner")]
    LoadingSpinner,
    #[at("/radius")]
    Radius,
    #[at("/size")]
    Size,
    #[at("/variant")]
    Variant,
    #[at("/variant-ghost")]
    VariantGhost,
    #[at("/with-icons")]
    WithIcons,
}

pub fn render(route: ButtonRoute) -> Html {
    match route {
        ButtonRoute::Root => html! { <button::ButtonExample />},
        ButtonRoute::Color => html! { <button_color::ButtonColorExample />},
        ButtonRoute::HighContrast => html! { <button_high_contrast::ButtonHighContrastExample />},
        ButtonRoute::Loading => html! { <button_loading::ButtonLoadingExample />},
        ButtonRoute::LoadingSpinner => {
            html! { <button_loading_spinner::ButtonLoadingSpinnerExample />}
        }
        ButtonRoute::Radius => html! { <button_radius::ButtonRadiusExample />},
        ButtonRoute::Size => html! { <button_size::ButtonSizeExample />},
        ButtonRoute::Variant => html! { <button_variant::ButtonVariantExample />},
        ButtonRoute::VariantGhost => html! { <button_variant_ghost::ButtonVariantGhostExample />},
        ButtonRoute::WithIcons => html! { <button_with_icons::ButtonWithIconsExample />},
    }
}
