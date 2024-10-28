use radix_yew_themes::{Code, Flex, FlexAlign, FlexDirection, Weight};
use yew::prelude::*;

#[function_component]
pub fn CodeWeightExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=2 align={FlexAlign::Start}>
            <Code weight={Weight::Light}>{"console.log()"}</Code>
            <Code weight={Weight::Regular}>{"console.log()"}</Code>
            <Code weight={Weight::Medium}>{"console.log()"}</Code>
            <Code weight={Weight::Bold}>{"console.log()"}</Code>
        </Flex>
    }
}
