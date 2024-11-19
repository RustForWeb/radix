use radix_yew_themes::{AccentColor, Badge, Flex};
use yew::prelude::*;

#[function_component]
pub fn BadgeColorExample() -> Html {
    html! {
        <Flex gap=2>
            <Badge color={AccentColor::Indigo}>{"New"}</Badge>
            <Badge color={AccentColor::Cyan}>{"New"}</Badge>
            <Badge color={AccentColor::Orange}>{"New"}</Badge>
            <Badge color={AccentColor::Crimson}>{"New"}</Badge>
        </Flex>
    }
}
