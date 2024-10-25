use radix_yew_themes::{AccentColor, Flex, FlexDirection, Heading};
use yew::prelude::*;

#[function_component]
pub fn HeadingColorExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column}>
            <Heading color={AccentColor::Indigo}>{"The quick brown fox jumps over the lazy dog"}</Heading>
            <Heading color={AccentColor::Cyan}>{"The quick brown fox jumps over the lazy dog"}</Heading>
            <Heading color={AccentColor::Orange}>{"The quick brown fox jumps over the lazy dog"}</Heading>
            <Heading color={AccentColor::Crimson}>{"The quick brown fox jumps over the lazy dog"}</Heading>
        </Flex>
    }
}
