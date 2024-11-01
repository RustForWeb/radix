use radix_yew_themes::{AccentColor, Flex, FlexDirection, TextField, TextFieldVariant};
use yew::prelude::*;

#[function_component]
pub fn TextFieldColorExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="250px">
            <TextField color={AccentColor::Indigo} variant={TextFieldVariant::Soft} placeholder="Search the docs…" />
            <TextField color={AccentColor::Green} variant={TextFieldVariant::Soft} placeholder="Search the docs…" />
            <TextField color={AccentColor::Red} variant={TextFieldVariant::Soft} placeholder="Search the docs…" />
        </Flex>
    }
}
