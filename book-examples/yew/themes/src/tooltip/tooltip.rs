use radix_yew_icons::PlusIcon;
use radix_yew_themes::{IconButton, Radius, Tooltip};
use yew::prelude::*;

#[function_component]
pub fn TooltipExample() -> Html {
    html! {
        <Tooltip content="Add to library">
            <IconButton radius={Radius::Full}>
                <PlusIcon />
            </IconButton>
        </Tooltip>
    }
}
