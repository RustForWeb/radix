use radix_yew_themes::Text;
use yew::prelude::*;

#[function_component]
pub fn TextExample() -> Html {
    html! {
        <Text>{"The quick brown fox jumps over the lazy dog."}</Text>
    }
}
