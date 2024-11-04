use radix_yew_icons::BookmarkIcon;
use radix_yew_themes::{Button, Spinner};
use yew::prelude::*;

#[function_component]
pub fn SpinnerWithButtonsDisabledExample() -> Html {
    html! {
        <Button disabled=true>
            <Spinner loading=true>
                <BookmarkIcon />
            </Spinner>
            {"Bookmark"}
        </Button>
    }
}
