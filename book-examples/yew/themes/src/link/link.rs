use radix_yew_themes::Link;
use yew::prelude::*;

#[function_component]
pub fn LinkExample() -> Html {
    html! {
        <Link href="#">{"Sign up"}</Link>
    }
}
