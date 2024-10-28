use radix_yew_themes::Code;
use yew::prelude::*;

#[function_component]
pub fn CodeExample() -> Html {
    html! {
        <Code>{"console.log()"}</Code>
    }
}
