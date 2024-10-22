use radix_yew_themes::{Flex, FlexAlign, Switch};
use yew::prelude::*;

#[function_component]
pub fn SwitchSizeExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=2>
            <Switch size=1 default_checked=true />
            <Switch size=2 default_checked=true />
            <Switch size=3 default_checked=true />
        </Flex>
    }
}
