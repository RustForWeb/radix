use radix_yew_themes::{Heading, Weight};
use yew::prelude::*;

#[function_component]
pub fn HeadingWeightExample() -> Html {
    html! {
        <>
            <Heading weight={Weight::Light}>
                {"The quick brown fox jumps over the lazy dog."}
            </Heading>

            <Heading weight={Weight::Regular}>
                {"The quick brown fox jumps over the lazy dog."}
            </Heading>

            <Heading weight={Weight::Medium}>
                {"The quick brown fox jumps over the lazy dog."}
            </Heading>

            <Heading weight={Weight::Bold}>
                {"The quick brown fox jumps over the lazy dog."}
            </Heading>
        </>
    }
}
