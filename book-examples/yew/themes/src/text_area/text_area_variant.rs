use radix_yew_themes::{Flex, FlexDirection, TextArea, TextAreaVariant};
use yew::prelude::*;

#[function_component]
pub fn TextAreaVariantExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="250px">
            <TextArea variant={TextAreaVariant::Surface} placeholder="Reply to comment…" />
            <TextArea variant={TextAreaVariant::Classic} placeholder="Reply to comment…" />
            <TextArea variant={TextAreaVariant::Soft} placeholder="Reply to comment…" />
        </Flex>
    }
}
