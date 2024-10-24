use radix_yew_themes::{AccentColor, Button, ButtonVariant, Flex, FlexDirection};
use yew::prelude::*;

#[function_component]
pub fn ButtonHighContrastExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Flex gap=3>
                <Button color={AccentColor::Gray} variant={ButtonVariant::Classic}>
                    {"Edit profile"}
                </Button>
                <Button color={AccentColor::Gray} variant={ButtonVariant::Solid}>
                    {"Edit profile"}
                </Button>
                <Button color={AccentColor::Gray} variant={ButtonVariant::Soft}>
                    {"Edit profile"}
                </Button>
                <Button color={AccentColor::Gray} variant={ButtonVariant::Surface}>
                    {"Edit profile"}
                </Button>
                <Button color={AccentColor::Gray} variant={ButtonVariant::Outline}>
                    {"Edit profile"}
                </Button>
            </Flex>
            <Flex gap=3>
                <Button color={AccentColor::Gray} variant={ButtonVariant::Classic} high_contrast=true>
                    {"Edit profile"}
                </Button>
                <Button color={AccentColor::Gray} variant={ButtonVariant::Solid} high_contrast=true>
                    {"Edit profile"}
                </Button>
                <Button color={AccentColor::Gray} variant={ButtonVariant::Soft} high_contrast=true>
                    {"Edit profile"}
                </Button>
                <Button color={AccentColor::Gray} variant={ButtonVariant::Surface} high_contrast=true>
                    {"Edit profile"}
                </Button>
                <Button color={AccentColor::Gray} variant={ButtonVariant::Outline} high_contrast=true>
                    {"Edit profile"}
                </Button>
            </Flex>
        </Flex>
    }
}
