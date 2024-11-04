use radix_yew_themes::{Flex, Radius, Switch};
use yew::prelude::*;

#[function_component]
pub fn SwitchRadiusExample() -> Html {
    html! {
        <Flex gap=3>
            <Switch radius={Radius::None} default_checked=true />
            <Switch radius={Radius::Small} default_checked=true />
            <Switch radius={Radius::Medium} default_checked=true />
            <Switch radius={Radius::Large} default_checked=true />
            <Switch radius={Radius::Full} default_checked=true />
        </Flex>
    }
}
