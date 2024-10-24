use radix_yew_themes::{Button, ButtonVariant, Flex, Radius};
use yew::prelude::*;

#[function_component]
pub fn ButtonRadiusExample() -> Html {
    html! {
        <Flex gap=3>
            <Button radius={Radius::None} variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
            <Button radius={Radius::Small} variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
            <Button radius={Radius::Medium} variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
            <Button radius={Radius::Large} variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
            <Button radius={Radius::Full} variant={ButtonVariant::Soft}>
                {"Edit profile"}
            </Button>
        </Flex>
    }
}
