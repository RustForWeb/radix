use radix_yew_themes::{Box, Flex, FlexDirection, TextField};
use yew::prelude::*;

#[function_component]
pub fn TextFieldSizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Box max_width="200px">
                <TextField size=1 placeholder="Search the docs…" />
            </Box>
            <Box max_width="250px">
                <TextField size=2 placeholder="Search the docs…" />
            </Box>
            <Box max_width="300px">
                <TextField size=3 placeholder="Search the docs…" />
            </Box>
        </Flex>
    }
}
