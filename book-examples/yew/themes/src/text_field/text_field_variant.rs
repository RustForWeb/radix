use radix_yew_themes::{Flex, FlexDirection, TextField, TextFieldVariant};
use yew::prelude::*;

#[function_component]
pub fn TextFieldVariantExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="250px">
            <TextField variant={TextFieldVariant::Surface} placeholder="Search the docs…" />
            <TextField variant={TextFieldVariant::Classic} placeholder="Search the docs…" />
            <TextField variant={TextFieldVariant::Soft} placeholder="Search the docs…" />
        </Flex>
    }
}
