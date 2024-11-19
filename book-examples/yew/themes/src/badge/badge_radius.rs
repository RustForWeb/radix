use radix_yew_themes::{AccentColor, Badge, BadgeVariant, Flex, Radius};
use yew::prelude::*;

#[function_component]
pub fn BadgeRadiusExample() -> Html {
    html! {
        <Flex gap=2>
            <Badge variant={BadgeVariant::Solid} radius={Radius::None} color={AccentColor::Indigo}>
                {"New"}
            </Badge>
            <Badge variant={BadgeVariant::Solid} radius={Radius::Small} color={AccentColor::Indigo}>
                {"New"}
            </Badge>
            <Badge variant={BadgeVariant::Solid} radius={Radius::Medium} color={AccentColor::Indigo}>
                {"New"}
            </Badge>
            <Badge variant={BadgeVariant::Solid} radius={Radius::Large} color={AccentColor::Indigo}>
                {"New"}
            </Badge>
            <Badge variant={BadgeVariant::Solid} radius={Radius::Full} color={AccentColor::Indigo}>
                {"New"}
            </Badge>
        </Flex>
    }
}
