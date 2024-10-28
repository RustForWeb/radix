use radix_yew_themes::{Em, Text};
use yew::prelude::*;

#[function_component]
pub fn EmExample() -> Html {
    html! {
        <Text>
            {"We "}<Em>{"had"}</Em>{" to do something about it."}
        </Text>
    }
}
