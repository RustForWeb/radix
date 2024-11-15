use radix_yew_themes::{Checkbox, CheckedState, Flex, FlexAs, FlexDirection, Text, TextAs};
use yew::prelude::*;

#[function_component]
pub fn CheckboxAlignmentExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Text r#as={TextAs::Label} size=2>
                <Flex r#as={FlexAs::Span} gap=2>
                    <Checkbox size=1 default_checked={CheckedState::True} />{" Agree to Terms and Conditions"}
                </Flex>
            </Text>

            <Text r#as={TextAs::Label} size=3>
                <Flex r#as={FlexAs::Span} gap=2>
                    <Checkbox size=2 default_checked={CheckedState::True} />{" Agree to Terms and Conditions"}
                </Flex>
            </Text>

            <Text r#as={TextAs::Label} size=4>
                <Flex r#as={FlexAs::Span} gap=2>
                    <Checkbox size=3 default_checked={CheckedState::True} />{" Agree to Terms and Conditions"}
                </Flex>
            </Text>
        </Flex>
    }
}
