use radix_yew_themes::{AccentColor, Flex, FlexDirection, Heading};
use yew::prelude::*;

#[function_component]
pub fn HeadingHighContrastExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column}>
            <Heading color={AccentColor::Gray}>{"The quick brown fox jumps over the lazy dog."}</Heading>
            <Heading color={AccentColor::Gray} high_contrast=true>
                {"The quick brown fox jumps over the lazy dog."}
            </Heading>
        </Flex>
    }
}
