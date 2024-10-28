use radix_yew_themes::{Strong, Text};
use yew::prelude::*;

#[function_component]
pub fn StrongExample() -> Html {
    html! {
        <Text>
            {"The most important thing to remember is, "}<Strong>{"stay positive"}</Strong>{"."}
        </Text>
    }
}
