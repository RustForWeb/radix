use radix_yew_themes::{Code, CodeVariant, Flex, FlexAlign, FlexDirection};
use yew::prelude::*;

#[function_component]
pub fn CodeVariantExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} align={FlexAlign::Start} gap=2>
            <Code variant={CodeVariant::Solid}>{"console.log()"}</Code>
            <Code variant={CodeVariant::Soft}>{"console.log()"}</Code>
            <Code variant={CodeVariant::Outline}>{"console.log()"}</Code>
            <Code variant={CodeVariant::Ghost}>{"console.log()"}</Code>
        </Flex>
    }
}
