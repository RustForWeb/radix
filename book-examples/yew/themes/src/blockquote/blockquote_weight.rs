use radix_yew_themes::{Blockquote, Flex, FlexDirection, Weight};
use yew::prelude::*;

#[function_component]
pub fn BlockquoteWeightExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="500px">
            <Blockquote weight={Weight::Light}>
                {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                stone alone comes near it in obstinacy."}
            </Blockquote>

            <Blockquote weight={Weight::Regular}>
                {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                stone alone comes near it in obstinacy."}
            </Blockquote>

            <Blockquote weight={Weight::Medium}>
                {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                stone alone comes near it in obstinacy."}
            </Blockquote>

            <Blockquote weight={Weight::Bold}>
                {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                stone alone comes near it in obstinacy."}
            </Blockquote>
        </Flex>
    }
}
