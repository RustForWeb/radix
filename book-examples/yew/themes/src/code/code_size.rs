use radix_yew_themes::{Code, Flex, FlexAlign, FlexDirection};
use yew::prelude::*;

#[function_component]
pub fn CodeSizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 align={FlexAlign::Start}>
            <Code size=1>{"console.log()"}</Code>
            <Code size=2>{"console.log()"}</Code>
            <Code size=3>{"console.log()"}</Code>
            <Code size=4>{"console.log()"}</Code>
            <Code size=5>{"console.log()"}</Code>
            <Code size=6>{"console.log()"}</Code>
            <Code size=7>{"console.log()"}</Code>
            <Code size=8>{"console.log()"}</Code>
            <Code size=9>{"console.log()"}</Code>
        </Flex>
    }
}
