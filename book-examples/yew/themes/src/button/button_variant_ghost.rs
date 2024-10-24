use radix_yew_themes::{Button, ButtonVariant};
use yew::prelude::*;

#[function_component]
pub fn ButtonVariantGhostExample() -> Html {
    html! {
        <Button variant={ButtonVariant::Ghost}>{"Edit profile"}</Button>
    }
}
