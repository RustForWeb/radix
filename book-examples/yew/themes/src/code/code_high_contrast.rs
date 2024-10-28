use radix_yew_themes::{AccentColor, Code, CodeVariant, Flex, FlexAlign, FlexDirection};
use yew::prelude::*;

#[function_component]
pub fn CodeHighContrastExample() -> Html {
    html! {
        <Flex gap=3>
            <Flex direction={FlexDirection::Column} align={FlexAlign::Start} gap=2>
                <Code color={AccentColor::Gray} variant={CodeVariant::Solid}>
                    {"console.log()"}
                </Code>
                <Code color={AccentColor::Gray} variant={CodeVariant::Soft}>
                    {"console.log()"}
                </Code>
                <Code color={AccentColor::Gray} variant={CodeVariant::Outline}>
                    {"console.log()"}
                </Code>
                <Code color={AccentColor::Gray} variant={CodeVariant::Ghost}>
                    {"console.log()"}
                </Code>
            </Flex>

            <Flex direction={FlexDirection::Column} align={FlexAlign::Start} gap=2>
                <Code color={AccentColor::Gray} variant={CodeVariant::Solid} high_contrast=true>
                    {"console.log()"}
                </Code>
                <Code color={AccentColor::Gray} variant={CodeVariant::Soft} high_contrast=true>
                    {"console.log()"}
                </Code>
                <Code color={AccentColor::Gray} variant={CodeVariant::Outline} high_contrast=true>
                    {"console.log()"}
                </Code>
                <Code color={AccentColor::Gray} variant={CodeVariant::Ghost} high_contrast=true>
                    {"console.log()"}
                </Code>
            </Flex>
        </Flex>
    }
}
