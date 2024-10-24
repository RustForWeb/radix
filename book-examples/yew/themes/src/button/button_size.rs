use radix_yew_themes::{Button, ButtonVariant, Flex, FlexAlign};
use yew::prelude::*;

#[function_component]
pub fn ButtonSizeExample() -> Html {
    html! {
        <Flex gap=3 align={FlexAlign::Center}>
            <Button size=1 variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
            <Button size=2 variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
            <Button size=3 variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
        </Flex>
    }
}
