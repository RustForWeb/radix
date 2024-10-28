use radix_yew_themes::{Box, Flex, FlexDirection, Heading, LeadingTrim, Text};
use yew::prelude::*;

#[function_component]
pub fn TextTrimBoxExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Box
                p=4
                style={[
                    ("background", "var(--gray-a2)"),
                    ("border", "1px dashed var(--gray-a7)"),
                ]}
            >
                <Heading mb=1 size=3>
                    {"Without trim"}
                </Heading>
                <Text>
                    {"The goal of typography is to relate font size, line height, and line width
                    in a proportional way that maximizes beauty and makes reading easier and
                    more pleasant."}
                </Text>
            </Box>

            <Box
                p=4
                style={[
                    ("background", "var(--gray-a2)"),
                    ("border", "1px dashed var(--gray-a7)"),
                ]}
            >
                <Heading mb=1 size=3 trim={LeadingTrim::Start}>
                    {"With trim"}
                </Heading>
                <Text trim={LeadingTrim::End}>
                    {"The goal of typography is to relate font size, line height, and line width
                    in a proportional way that maximizes beauty and makes reading easier and
                    more pleasant."}
                </Text>
            </Box>
        </Flex>
    }
}
