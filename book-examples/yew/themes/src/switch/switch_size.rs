use radix_yew_themes::{Switch, SwitchSize};
use yew::prelude::*;

#[function_component]
pub fn SwitchSizeExample() -> Html {
    html! {
        // <Flex align="center" gap="2">
        <>
            <Switch size={SwitchSize::S1} default_checked=true />
            <Switch size={SwitchSize::S2} default_checked=true />
            <Switch size={SwitchSize::S3} default_checked=true />
        </>
        // </Flex>
    }
}
