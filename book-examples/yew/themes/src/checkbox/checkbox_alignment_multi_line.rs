use radix_yew_themes::{Box, Checkbox, CheckedState, Flex, FlexAs, Text, TextAs};
use yew::prelude::*;

#[function_component]
pub fn CheckboxAlignmentMultiLineExample() -> Html {
    html! {
        <Box max_width="300px">
            <Text r#as={TextAs::Label} size=3>
                <Flex r#as={FlexAs::Span} gap=2>
                    <Checkbox default_checked={CheckedState::True} />{" I understand that these documents are
                    confidential and cannot be shared with a third party."}
                </Flex>
            </Text>
        </Box>
    }
}
