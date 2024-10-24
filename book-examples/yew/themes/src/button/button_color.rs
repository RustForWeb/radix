use radix_yew_themes::{AccentColor, Button, ButtonVariant, Flex};
use yew::prelude::*;

#[function_component]
pub fn ButtonColorExample() -> Html {
    html! {
        <Flex gap=3>
            <Button color={AccentColor::Indigo} variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
            <Button color={AccentColor::Cyan} variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
            <Button color={AccentColor::Orange} variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
            <Button color={AccentColor::Crimson} variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
        </Flex>
    }
}
