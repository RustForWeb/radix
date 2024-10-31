use radix_yew_themes::{Avatar, AvatarVariant, Flex};
use yew::prelude::*;

#[function_component]
pub fn AvatarVariantExample() -> Html {
    html! {
        <Flex gap=2>
            <Avatar variant={AvatarVariant::Solid} fallback="A" />
            <Avatar variant={AvatarVariant::Soft} fallback="A" />
        </Flex>
    }
}
