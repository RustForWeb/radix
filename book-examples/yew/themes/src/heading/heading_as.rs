use radix_yew_themes::{Heading, HeadingAs};
use yew::prelude::*;

#[function_component]
pub fn HeadingAsExample() -> Html {
    html! {
        <>
            <Heading r#as={HeadingAs::H1}>{"Level 1"}</Heading>
            <Heading r#as={HeadingAs::H2}>{"Level 2"}</Heading>
            <Heading r#as={HeadingAs::H3}>{"Level 3"}</Heading>
        </>
    }
}
