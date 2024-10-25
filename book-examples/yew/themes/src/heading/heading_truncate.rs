use radix_yew_themes::{Flex, Heading};
use yew::prelude::*;

#[function_component]
pub fn HeadingTruncateExample() -> Html {
    html! {
        <Flex max_width="300px">
            <Heading truncate=true>{"The quick brown fox jumps over the lazy dog"}</Heading>
        </Flex>
    }
}
