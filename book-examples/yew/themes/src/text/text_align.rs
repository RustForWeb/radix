use radix_yew_themes::{Text, TextAlign, TextAs};
use yew::prelude::*;

#[function_component]
pub fn TextAlignExample() -> Html {
    html! {
        <>
            <Text align={TextAlign::Left} r#as={TextAs::Div}>{"Left-aligned"}</Text>
            <Text align={TextAlign::Center} r#as={TextAs::Div}>{"Center-aligned"}</Text>
            <Text align={TextAlign::Right} r#as={TextAs::Div}>{"Right-aligned"}</Text>
        </>
    }
}
