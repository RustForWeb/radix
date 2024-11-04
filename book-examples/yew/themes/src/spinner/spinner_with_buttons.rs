use radix_yew_themes::Button;
use yew::prelude::*;

#[function_component]
pub fn SpinnerWithButtonsExample() -> Html {
    html! {
        <Button loading=true>{"Bookmark"}</Button>
    }
}
