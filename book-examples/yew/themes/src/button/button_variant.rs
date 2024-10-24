use radix_yew_themes::{Button, ButtonVariant, Flex, FlexAlign};
use yew::prelude::*;

#[function_component]
pub fn ButtonVariantExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=3>
            <Button variant={ButtonVariant::Classic}>{"Edit profile"}</Button>
            <Button variant={ButtonVariant::Solid}>{"Edit profile"}</Button>
            <Button variant={ButtonVariant::Soft}>{"Edit profile"}</Button>
            <Button variant={ButtonVariant::Surface}>{"Edit profile"}</Button>
            <Button variant={ButtonVariant::Outline}>{"Edit profile"}</Button>
        </Flex>
    }
}
