use radix_yew_themes::{AccentColor, Badge, BadgeVariant, Flex, FlexDirection};
use yew::prelude::*;

#[function_component]
pub fn BadgeHighContrastExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=2>
            <Flex gap=2>
                <Badge color={AccentColor::Gray} variant={BadgeVariant::Solid}>
                    {"New"}
                </Badge>
                <Badge color={AccentColor::Gray} variant={BadgeVariant::Soft}>
                    {"New"}
                </Badge>
                <Badge color={AccentColor::Gray} variant={BadgeVariant::Surface}>
                    {"New"}
                </Badge>
                <Badge color={AccentColor::Gray} variant={BadgeVariant::Outline}>
                    {"New"}
                </Badge>
                </Flex>
                <Flex gap="2">
                <Badge color={AccentColor::Gray} variant={BadgeVariant::Solid} high_contrast=true>
                    {"New"}
                </Badge>
                <Badge color={AccentColor::Gray} variant={BadgeVariant::Soft} high_contrast=true>
                    {"New"}
                </Badge>
                <Badge color={AccentColor::Gray} variant={BadgeVariant::Surface} high_contrast=true>
                    {"New"}
                </Badge>
                <Badge color={AccentColor::Gray} variant={BadgeVariant::Outline} high_contrast=true>
                    {"New"}
                </Badge>
            </Flex>
        </Flex>
    }
}
