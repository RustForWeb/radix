use radix_yew_themes::{Flex, FlexDirection, TextArea, TextAreaResize};
use yew::prelude::*;

#[function_component]
pub fn TextAreaResizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="250px">
            <TextArea resize={TextAreaResize::None} placeholder="Reply to comment…" />
            <TextArea resize={TextAreaResize::Vertical} placeholder="Reply to comment…" />
            <TextArea resize={TextAreaResize::Horizontal} placeholder="Reply to comment…" />
            <TextArea resize={TextAreaResize::Both} placeholder="Reply to comment…" />
        </Flex>
    }
}
