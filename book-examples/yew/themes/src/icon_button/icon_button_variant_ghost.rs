use radix_yew_icons::MagnifyingGlassIcon;
use radix_yew_themes::{IconButton, IconButtonVariant};
use yew::prelude::*;

#[function_component]
pub fn IconButtonVariantGhostExample() -> Html {
    html! {
        <IconButton variant={IconButtonVariant::Ghost}>
            <MagnifyingGlassIcon width=18 height=18 />
        </IconButton>
    }
}
