use radix_yew_themes::{Quote, Text};
use yew::prelude::*;

#[function_component]
pub fn QuoteExample() -> Html {
    html! {
        <Text>
            {"His famous quote, "}
            <Quote>{"Styles come and go. Good design is a language, not a style"}</Quote>{",
            elegantly summs up Massimo's philosophy of design."}
        </Text>
    }
}
