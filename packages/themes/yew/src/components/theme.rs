use yew::prelude::*;

use crate::components::theme_props::Appearance;

#[derive(PartialEq, Properties)]
pub struct ThemeProps {
    appearance: Appearance,
}

#[function_component]
pub fn Theme(props: &ThemeProps) -> Html {
    html! {}
}
