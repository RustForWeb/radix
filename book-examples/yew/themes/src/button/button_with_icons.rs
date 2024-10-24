use radix_yew_icons::BookmarkIcon;
use radix_yew_themes::{Button, ButtonVariant, Flex};
use yew::prelude::*;

#[function_component]
pub fn ButtonWithIconsExample() -> Html {
    html! {
        <Flex gap=3>
            <Button variant={ButtonVariant::Classic}>
                <BookmarkIcon /> {"Bookmark"}
            </Button>
            <Button variant={ButtonVariant::Solid}>
                <BookmarkIcon /> {"Bookmark"}
            </Button>
            <Button variant={ButtonVariant::Soft}>
                <BookmarkIcon /> {"Bookmark"}
            </Button>
            <Button variant={ButtonVariant::Surface}>
                <BookmarkIcon /> {"Bookmark"}
            </Button>
            <Button variant={ButtonVariant::Outline}>
                <BookmarkIcon /> {"Bookmark"}
            </Button>
        </Flex>
    }
}
