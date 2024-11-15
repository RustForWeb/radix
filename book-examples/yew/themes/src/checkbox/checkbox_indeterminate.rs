use radix_yew_themes::{Checkbox, CheckedState, Flex};
use yew::prelude::*;

#[function_component]
pub fn CheckboxIndeterminateExample() -> Html {
    html! {
        <Flex gap=2>
            <Checkbox default_checked={CheckedState::Indeterminate} />
            <Checkbox checked={CheckedState::Indeterminate} />
        </Flex>
    }
}
