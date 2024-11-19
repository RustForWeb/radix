use radix_yew_themes::{AccentColor, Badge, BadgeVariant, Flex};
use yew::prelude::*;

#[function_component]
pub fn BadgeVariantExample() -> Html {
    html! {
        <Flex gap=2>
            <Badge variant={BadgeVariant::Solid} color={AccentColor::Indigo}>
                {"New"}
            </Badge>
            <Badge variant={BadgeVariant::Soft} color={AccentColor::Indigo}>
                {"New"}
            </Badge>
            <Badge variant={BadgeVariant::Surface} color={AccentColor::Indigo}>
                {"New"}
            </Badge>
            <Badge variant={BadgeVariant::Outline} color={AccentColor::Indigo}>
                {"New"}
            </Badge>
        </Flex>
    }
}
