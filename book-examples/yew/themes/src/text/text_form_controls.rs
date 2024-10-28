use radix_yew_themes::{Box, Flex, Text, TextAs};
use yew::prelude::*;

// TODO

#[function_component]
pub fn TextFormControlsExample() -> Html {
    html! {
        <>
            <Box max_width="300px">
                <Text r#as={TextAs::Label} size=3>
                    <Flex gap=2>
                        /*<Checkbox default_checked=true />*/{" I understand that these documents are
                        confidential and cannot be shared with a third party."}
                    </Flex>
                </Text>
            </Box>
        </>
    }
}
