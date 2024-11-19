use radix_yew_themes::{AccentColor, Badge, Flex, FlexAlign};
use yew::prelude::*;

#[function_component]
pub fn BadgeSizeExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=2>
            <Badge size=1 color={AccentColor::Indigo}>
                {"New"}
            </Badge>
            <Badge size=2 color={AccentColor::Indigo}>
                {"New"}
            </Badge>
            <Badge size=3 color={AccentColor::Indigo}>
                {"New"}
            </Badge>
        </Flex>
    }
}
