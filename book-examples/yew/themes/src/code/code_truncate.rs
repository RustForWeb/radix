use radix_yew_themes::{Code, Flex};
use yew::prelude::*;

#[function_component]
pub fn CodeTruncateExample() -> Html {
    html! {
        <Flex max_width="200px">
            <Code truncate=true>{"linear-gradient(red, orange, yellow, green, blue);"}</Code>
        </Flex>
    }
}
