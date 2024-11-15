use radix_yew_themes::{Checkbox, CheckedState, Flex, FlexAlign};
use yew::prelude::*;

#[function_component]
pub fn CheckboxSizeExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=2>
            <Checkbox size=1 default_checked={CheckedState::True} />
            <Checkbox size=2 default_checked={CheckedState::True} />
            <Checkbox size=3 default_checked={CheckedState::True} />
        </Flex>
    }
}
