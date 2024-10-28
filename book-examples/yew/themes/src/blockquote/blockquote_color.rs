use radix_yew_themes::{AccentColor, Blockquote, Flex, FlexDirection};
use yew::prelude::*;

#[function_component]
pub fn BlockquoteColorExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="500px">
            <Blockquote color={AccentColor::Indigo}>
                {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                stone alone comes near it in obstinacy."}
            </Blockquote>

            <Blockquote color={AccentColor::Cyan}>
                {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                stone alone comes near it in obstinacy."}
            </Blockquote>

            <Blockquote color={AccentColor::Orange}>
                {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                stone alone comes near it in obstinacy."}
            </Blockquote>

            <Blockquote color={AccentColor::Crimson}>
                {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                stone alone comes near it in obstinacy."}
            </Blockquote>
        </Flex>
    }
}
