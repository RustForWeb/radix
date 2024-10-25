use radix_yew_themes::{Flex, FlexDirection, Heading, TextWrap};
use yew::prelude::*;

#[function_component]
pub fn HeadingWrapExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Flex max_width="300px">
                <Heading wrap={TextWrap::Nowrap}>{"The quick brown fox jumps over the lazy dog"}</Heading>
            </Flex>

            <Flex max_width="300px">
                <Heading wrap={TextWrap::Balance}>{"The quick brown fox jumps over the lazy dog"}</Heading>
            </Flex>

            <Flex max_width="300px">
                <Heading wrap={TextWrap::Pretty}>{"The quick brown fox jumps over the lazy dog"}</Heading>
            </Flex>
        </Flex>
    }
}
