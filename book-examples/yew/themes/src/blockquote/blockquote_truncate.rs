use radix_yew_themes::{Blockquote, Flex};
use yew::prelude::*;

#[function_component]
pub fn BlockquoteTruncateExample() -> Html {
    html! {
        <Flex max_width="500px">
            <Blockquote truncate=true>
                {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                stone alone comes near it in obstinacy."}
            </Blockquote>
        </Flex>
    }
}
