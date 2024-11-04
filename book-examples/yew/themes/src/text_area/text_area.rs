use radix_yew_themes::TextArea;
use yew::prelude::*;

#[function_component]
pub fn TextAreaExample() -> Html {
    html! {
        <TextArea placeholder="Reply to comment…" />
    }
}
