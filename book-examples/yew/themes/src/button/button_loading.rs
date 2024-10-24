use radix_yew_themes::{Button, ButtonVariant, Flex};
use yew::prelude::*;

#[function_component]
pub fn ButtonLoadingExample() -> Html {
    html! {
        <Flex gap=3>
            <Button loading=true variant={ButtonVariant::Classic}>
                {"Bookmark"}
            </Button>
            <Button loading=true variant={ButtonVariant::Solid}>
                {"Bookmark"}
            </Button>
            <Button loading=true variant={ButtonVariant::Soft}>
                {"Bookmark"}
            </Button>
            <Button loading=true variant={ButtonVariant::Surface}>
                {"Bookmark"}
            </Button>
            <Button loading=true variant={ButtonVariant::Outline}>
                {"Bookmark"}
            </Button>
        </Flex>
    }
}
