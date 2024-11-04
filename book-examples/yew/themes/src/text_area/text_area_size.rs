use radix_yew_themes::{Box, Flex, FlexDirection, TextArea};
use yew::prelude::*;

#[function_component]
pub fn TextAreaSizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Box max_width="200px">
                <TextArea size=1 placeholder="Reply to comment…" />
            </Box>
            <Box max_width="250px">
                <TextArea size=2 placeholder="Reply to comment…" />
            </Box>
            <Box max_width="300px">
                <TextArea size=3 placeholder="Reply to comment…" />
            </Box>
        </Flex>
    }
}
