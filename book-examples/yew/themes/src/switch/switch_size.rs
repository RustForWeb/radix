use radix_yew_themes::{Flex, FlexAlign, Switch, SwitchSize};
use yew::prelude::*;

#[function_component]
pub fn SwitchSizeExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=2>
            <Switch size={SwitchSize::S1} default_checked=true />
            <Switch size={SwitchSize::S2} default_checked=true />
            <Switch size={SwitchSize::S3} default_checked=true />
        </Flex>
    }
}
