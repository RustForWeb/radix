use radix_yew_themes::Blockquote;
use yew::prelude::*;

#[function_component]
pub fn BlockquoteExample() -> Html {
    html! {
        <Blockquote>
            {"Perfect typography is certainly the most elusive of all arts. Sculpture in
            stone alone comes near it in obstinacy."}
        </Blockquote>
    }
}
