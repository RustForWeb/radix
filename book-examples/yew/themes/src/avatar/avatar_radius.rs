use radix_yew_themes::{Avatar, Flex, Radius};
use yew::prelude::*;

#[function_component]
pub fn AvatarRadiusExample() -> Html {
    html! {
        <Flex gap=2>
            <Avatar radius={Radius::None} fallback="A" />
            <Avatar radius={Radius::Large} fallback="A" />
            <Avatar radius={Radius::Full} fallback="A" />
        </Flex>
    }
}
