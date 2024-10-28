use radix_yew_themes::Kbd;
use yew::prelude::*;

#[function_component]
pub fn KbdExample() -> Html {
    html! {
        <Kbd>{"Shift + Tab"}</Kbd>
    }
}
