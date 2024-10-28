use radix_yew_themes::{Em, Kbd, Text, TextAs};
use yew::prelude::*;

// TODO

#[function_component]
pub fn TextFormattingExample() -> Html {
    html! {
        <Text r#as={TextAs::P}>
            {"Look, such a helpful "}/*<Link href="#">{"link"}</Link>*/{", an "}<Em>{"italic emphasis"}</Em>{",
            a piece of computer "}/*<Code>{"code"}</Code>*/{", and even a hotkey combination "}
            <Kbd>{"⇧⌘A"}</Kbd>{" within the text."}
        </Text>
    }
}
