use radix_yew_themes::{Checkbox, CheckboxVariant, CheckedState, Flex, FlexAlign};
use yew::prelude::*;

#[function_component]
pub fn CheckboxVariantExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=4>
            <Flex gap=2>
                <Checkbox variant={CheckboxVariant::Surface} default_checked={CheckedState::True} />
                <Checkbox variant={CheckboxVariant::Surface} />
            </Flex>

            <Flex gap=2>
                <Checkbox variant={CheckboxVariant::Classic} default_checked={CheckedState::True} />
                <Checkbox variant={CheckboxVariant::Classic} />
            </Flex>

            <Flex gap=2>
                <Checkbox variant={CheckboxVariant::Soft} default_checked={CheckedState::True} />
                <Checkbox variant={CheckboxVariant::Soft} />
            </Flex>
        </Flex>
    }
}
