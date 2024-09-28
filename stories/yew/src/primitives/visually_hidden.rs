use radix_yew_visually_hidden::*;
use yew::prelude::*;

#[function_component]
pub fn Basic() -> Html {
    html! {
        <button>
            <VisuallyHidden>{"Save the file"}</VisuallyHidden>
            <span aria-hidden="true">{"💾"}</span>
        </button>
    }
}
