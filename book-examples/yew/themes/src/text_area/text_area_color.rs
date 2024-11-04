use radix_yew_themes::{AccentColor, Flex, FlexDirection, TextArea, TextAreaVariant};
use yew::prelude::*;

#[function_component]
pub fn TextAreaColorExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="250px">
            <TextArea color={AccentColor::Indigo} variant={TextAreaVariant::Soft} placeholder="Reply to comment…" />
            <TextArea color={AccentColor::Green} variant={TextAreaVariant::Soft} placeholder="Reply to comment…" />
            <TextArea color={AccentColor::Red} variant={TextAreaVariant::Soft} placeholder="Reply to comment…" />
        </Flex>
    }
}
