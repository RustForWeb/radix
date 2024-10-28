use radix_yew_themes::{Flex, FlexAlign, FlexDirection, Kbd};
use yew::prelude::*;

#[function_component]
pub fn KbdSizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} align={FlexAlign::Start} gap=3>
            <Kbd size=1>{"Shift + Tab"}</Kbd>
            <Kbd size=2>{"Shift + Tab"}</Kbd>
            <Kbd size=3>{"Shift + Tab"}</Kbd>
            <Kbd size=4>{"Shift + Tab"}</Kbd>
            <Kbd size=5>{"Shift + Tab"}</Kbd>
            <Kbd size=6>{"Shift + Tab"}</Kbd>
            <Kbd size=7>{"Shift + Tab"}</Kbd>
            <Kbd size=8>{"Shift + Tab"}</Kbd>
            <Kbd size=9>{"Shift + Tab"}</Kbd>
        </Flex>
    }
}
