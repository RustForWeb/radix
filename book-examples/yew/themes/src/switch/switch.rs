use radix_yew_themes::Switch;
use yew::prelude::*;

#[function_component]
pub fn SwitchExample() -> Html {
    html! {
        <Switch default_checked=true />
    }
}
