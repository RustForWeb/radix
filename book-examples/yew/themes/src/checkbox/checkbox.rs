use radix_yew_themes::{Checkbox, CheckedState, Flex, Text, TextAs};
use yew::prelude::*;

#[function_component]
pub fn CheckboxExample() -> Html {
    html! {
        <Text r#as={TextAs::Label} size=2>
            <Flex gap=2>
                <Checkbox default_checked={CheckedState::True} />
                {"Agree to Terms and Conditions"}
            </Flex>
        </Text>
    }
}
