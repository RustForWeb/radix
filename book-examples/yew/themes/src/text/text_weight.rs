use radix_yew_themes::{Text, TextAs, Weight};
use yew::prelude::*;

#[function_component]
pub fn TextWeightExample() -> Html {
    html! {
        <>
            <Text weight={Weight::Light} r#as={TextAs::Div} >
                {"The quick brown fox jumps over the lazy dog."}
            </Text>

            <Text weight={Weight::Regular} r#as={TextAs::Div} >
                {"The quick brown fox jumps over the lazy dog."}
            </Text>

            <Text weight={Weight::Medium} r#as={TextAs::Div} >
                {"The quick brown fox jumps over the lazy dog."}
            </Text>

            <Text weight={Weight::Bold} r#as={TextAs::Div} >
                {"The quick brown fox jumps over the lazy dog."}
            </Text>
        </>
    }
}
