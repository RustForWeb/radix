use radix_yew_icons::BookmarkIcon;
use radix_yew_themes::Button;
use yew::prelude::*;

#[function_component]
pub fn ButtonExample() -> Html {
    html! {
        <Button>
            <BookmarkIcon /> {"Bookmark"}
        </Button>
    }
}
