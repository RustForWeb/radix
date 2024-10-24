use radix_yew_themes::{Button, ButtonVariant, Flex};
use yew::prelude::*;

// TODO

#[function_component]
pub fn ButtonLoadingSpinnerExample() -> Html {
    html! {
        <Flex gap=3>
            <Button disabled=true variant={ButtonVariant::Classic}>
                // <Spinner loading=true>
                //     <BookmarkIcon />
                // </Spinner>
                {"Bookmark"}
            </Button>
            <Button disabled=true variant={ButtonVariant::Solid}>
                // <Spinner loading=true>
                //     <BookmarkIcon />
                // </Spinner>
                {"Bookmark"}
            </Button>
            <Button disabled=true variant={ButtonVariant::Soft}>
                // <Spinner loading=true>
                //     <BookmarkIcon />
                // </Spinner>
                {"Bookmark"}
            </Button>
            <Button disabled=true variant={ButtonVariant::Surface}>
                // <Spinner loading=true>
                //     <BookmarkIcon />
                // </Spinner>
                {"Bookmark"}
            </Button>
            <Button disabled=true variant={ButtonVariant::Outline}>
                // <Spinner loading=true>
                //     <BookmarkIcon />
                // </Spinner>
                {"Bookmark"}
            </Button>
        </Flex>
    }
}
