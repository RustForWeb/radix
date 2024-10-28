use radix_yew_themes::{AccentColor, Code, Flex, FlexAlign, FlexDirection};
use yew::prelude::*;

#[function_component]
pub fn CodeColorExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} align={FlexAlign::Start} gap=2>
            <Code color={AccentColor::Indigo}>{"console.log()"}</Code>
            <Code color={AccentColor::Crimson}>{"console.log()"}</Code>
            <Code color={AccentColor::Cyan}>{"console.log()"}</Code>
            <Code color={AccentColor::Orange}>{"console.log()"}</Code>
        </Flex>
    }
}
