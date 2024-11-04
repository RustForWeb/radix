use radix_yew_themes::{Flex, FlexDirection, Radius, TextArea};
use yew::prelude::*;

#[function_component]
pub fn TextAreaRadiusExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="250px">
            <TextArea radius={Radius::None} placeholder="Reply to comment…" />
            <TextArea radius={Radius::Large} placeholder="Reply to comment…" />
            <TextArea radius={Radius::Full} placeholder="Reply to comment…" />
        </Flex>
    }
}
