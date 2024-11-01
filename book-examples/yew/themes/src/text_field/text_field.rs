use radix_yew_icons::MagnifyingGlassIcon;
use radix_yew_themes::{TextField, TextFieldSlot};
use yew::prelude::*;

#[function_component]
pub fn TextFieldExample() -> Html {
    html! {
        <TextField placeholder="Search the docs…">
            <TextFieldSlot>
                <MagnifyingGlassIcon height="16" width="16" />
            </TextFieldSlot>
        </TextField>
    }
}
