use radix_yew_themes::{AccentColor, Avatar, AvatarVariant, Flex};
use yew::prelude::*;

#[function_component]
pub fn AvatarColorExample() -> Html {
    html! {
        <Flex gap=2>
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Indigo} fallback="A" />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Cyan} fallback="A" />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Orange} fallback="A" />
            <Avatar variant={AvatarVariant::Solid} color={AccentColor::Crimson} fallback="A" />
        </Flex>
    }
}
