use radix_yew_themes::{AccentColor, Blockquote, Flex, FlexDirection};
use yew::prelude::*;

#[function_component]
pub fn BlockquoteHighContrastExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="500px">
            <Blockquote color={AccentColor::Gray}>
                {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                stone alone comes near it in obstinacy."}
            </Blockquote>
            <Blockquote color={AccentColor::Gray} high_contrast=true>
                {"Perfect typography is certainly the most elusive of all arts. Sculpture in
                stone alone comes near it in obstinacy."}
            </Blockquote>
        </Flex>
    }
}
