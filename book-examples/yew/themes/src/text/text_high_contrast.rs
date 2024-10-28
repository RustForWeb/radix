use radix_yew_themes::{AccentColor, Flex, FlexDirection, Text};
use yew::prelude::*;

#[function_component]
pub fn TextHighContrastExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column}>
            <Text color={AccentColor::Gray}>{"The quick brown fox jumps over the lazy dog."}</Text>
            <Text color={AccentColor::Gray} high_contrast=true>
                {"The quick brown fox jumps over the lazy dog."}
            </Text>
        </Flex>
    }
}
