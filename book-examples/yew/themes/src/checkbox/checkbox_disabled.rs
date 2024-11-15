use radix_yew_themes::{
    AccentColor, Checkbox, CheckedState, Flex, FlexAs, FlexDirection, Text, TextAs,
};
use yew::prelude::*;

#[function_component]
pub fn CheckboxDisabledExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=2>
            <Text r#as={TextAs::Label} size=2>
                <Flex r#as={FlexAs::Span} gap=2>
                    <Checkbox />
                    {"Not checked"}
                </Flex>
            </Text>

            <Text r#as={TextAs::Label} size=2>
                <Flex r#as={FlexAs::Span} gap=2>
                    <Checkbox default_checked={CheckedState::True} />
                    {"Checked"}
                </Flex>
            </Text>

            <Text r#as={TextAs::Label} size=2 color={AccentColor::Gray}>
                <Flex r#as={FlexAs::Span} gap=2>
                    <Checkbox disabled=true />
                    {"Not checked"}
                </Flex>
            </Text>

            <Text r#as={TextAs::Label} size=2 color={AccentColor::Gray}>
                <Flex r#as={FlexAs::Span} gap=2>
                    <Checkbox disabled=true default_checked={CheckedState::True} />
                    {"Checked"}
                </Flex>
            </Text>
        </Flex>
    }
}
