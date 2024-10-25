use radix_yew_themes::Heading;
use yew::prelude::*;

#[function_component]
pub fn HeadingExample() -> Html {
    html! {
        <Heading>{"The quick brown fox jumps over the lazy dog"}</Heading>
    }
}
