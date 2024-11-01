use radix_yew_themes::{Flex, FlexDirection, Radius, TextField};
use yew::prelude::*;

#[function_component]
pub fn TextFieldRadiusExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="250px">
            <TextField radius={Radius::None} placeholder="Search the docs…" />
            <TextField radius={Radius::Large} placeholder="Search the docs…" />
            <TextField radius={Radius::Full} placeholder="Search the docs…" />
        </Flex>
    }
}
