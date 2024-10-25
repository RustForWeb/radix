use radix_yew_themes::{Heading, TextAlign};
use yew::prelude::*;

#[function_component]
pub fn HeadingAlignExample() -> Html {
    html! {
        <>
            <Heading align={TextAlign::Left}>{"Left-aligned"}</Heading>
            <Heading align={TextAlign::Center}>{"Center-aligned"}</Heading>
            <Heading align={TextAlign::Right}>{"Right-aligned"}</Heading>
        </>
    }
}
