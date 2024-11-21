use radix_yew_themes::{Flex, FlexAlign, Radio};
use yew::prelude::*;

#[function_component]
pub fn RadioSizeExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=4>
            <Flex gap=2>
                <Radio size=1 name="size-1" value="1" default_checked=true />
                <Radio size=1 name="size-1" value="2" />
            </Flex>

            <Flex gap=2>
                <Radio size=2 name="size-2" value="1" default_checked=true />
                <Radio size=2 name="size-2" value="2" />
            </Flex>

            <Flex gap=2>
                <Radio size=3 name="size-3" value="1" default_checked=true />
                <Radio size=3 name="size-3" value="2" />
            </Flex>
        </Flex>
    }
}
