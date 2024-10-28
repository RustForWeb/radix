use radix_yew_themes::{Strong, Text, TextAs};
use yew::prelude::*;

#[function_component]
pub fn TextAsExample() -> Html {
    html! {
        <>
            <Text r#as={TextAs::P}>{"This is a "}<Strong>{"paragraph"}</Strong>{" element."}</Text>
            <Text r#as={TextAs::Label}>{"This is a "}<Strong>{"label"}</Strong>{" element."}</Text>
            <Text r#as={TextAs::Div}>{"This is a "}<Strong>{"div"}</Strong>{" element."}</Text>
            <Text r#as={TextAs::Span}>{"This is a "}<Strong>{"span"}</Strong>{" element."}</Text>
        </>
    }
}
