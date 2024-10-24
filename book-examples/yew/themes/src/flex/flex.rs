use radix_yew_themes::{Box, Flex};
use yew::prelude::*;

use crate::decorative_box::DecorativeBox;

#[function_component]
pub fn FlexExample() -> Html {
    html! {
        <Flex gap=3>
            <Box width="64px" height="64px">
                <DecorativeBox />
            </Box>
            <Box width="64px" height="64px">
                <DecorativeBox />
            </Box>
            <Box width="64px" height="64px">
                <DecorativeBox />
            </Box>
            <Box width="64px" height="64px">
                <DecorativeBox />
            </Box>
            <Box width="64px" height="64px">
                <DecorativeBox />
            </Box>
        </Flex>
    }
}
