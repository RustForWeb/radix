use radix_yew_themes::{AccentColor, Flex, FlexDirection, Text};
use yew::prelude::*;

#[function_component]
pub fn TextColorExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column}>
            <Text color={AccentColor::Indigo}>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text color={AccentColor::Cyan}>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text color={AccentColor::Orange}>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text color={AccentColor::Crimson}>{"The quick brown fox jumps over the lazy dog."}</Text>
        </Flex>
    }
}
