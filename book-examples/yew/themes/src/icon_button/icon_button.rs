use radix_yew_icons::MagnifyingGlassIcon;
use radix_yew_themes::IconButton;
use yew::prelude::*;

#[function_component]
pub fn IconButtonExample() -> Html {
    html! {
        <IconButton>
            <MagnifyingGlassIcon width=18 height=18 />
        </IconButton>
    }
}
