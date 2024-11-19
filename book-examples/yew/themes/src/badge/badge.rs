use radix_yew_themes::{AccentColor, Badge, Flex};
use yew::prelude::*;

#[function_component]
pub fn BadgeExample() -> Html {
    html! {
        <Flex gap=2>
            <Badge color={AccentColor::Orange}>{"In progress"}</Badge>
            <Badge color={AccentColor::Blue}>{"In review"}</Badge>
            <Badge color={AccentColor::Green}>{"Complete"}</Badge>
        </Flex>
    }
}
