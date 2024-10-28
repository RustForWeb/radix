use radix_yew_themes::{Flex, FlexDirection, Text};
use yew::prelude::*;

#[function_component]
pub fn TextSizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Text size=1>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text size=2>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text size=3>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text size=4>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text size=5>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text size=6>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text size=7>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text size=8>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text size=9>{"The quick brown fox jumps over the lazy dog."}</Text>
        </Flex>
    }
}
