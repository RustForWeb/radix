use radix_yew_themes::{Flex, FlexDirection, Heading};
use yew::prelude::*;

#[function_component]
pub fn HeadingSizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Heading size=1>{"The quick brown fox jumps over the lazy dog"}</Heading>
            <Heading size=2>{"The quick brown fox jumps over the lazy dog"}</Heading>
            <Heading size=3>{"The quick brown fox jumps over the lazy dog"}</Heading>
            <Heading size=4>{"The quick brown fox jumps over the lazy dog"}</Heading>
            <Heading size=5>{"The quick brown fox jumps over the lazy dog"}</Heading>
            <Heading size=6>{"The quick brown fox jumps over the lazy dog"}</Heading>
            <Heading size=7>{"The quick brown fox jumps over the lazy dog"}</Heading>
            <Heading size=8>{"The quick brown fox jumps over the lazy dog"}</Heading>
            <Heading size=9>{"The quick brown fox jumps over the lazy dog"}</Heading>
        </Flex>
    }
}
