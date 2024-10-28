use radix_yew_themes::{Flex, FlexDirection, LeadingTrim, Text};
use yew::prelude::*;

#[function_component]
pub fn TextTrimExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Text
                trim={LeadingTrim::Normal}
                style={[
                    ("background", "var(--gray-a2)"),
                    ("border-top", "1px dashed var(--gray-a7)"),
                    ("border-bottom", "1px dashed var(--gray-a7)"),
                ]}
            >
                {"Without trim"}
            </Text>
            <Text
                trim={LeadingTrim::Both}
                style={[
                    ("background", "var(--gray-a2)"),
                    ("border-top", "1px dashed var(--gray-a7)"),
                    ("border-bottom", "1px dashed var(--gray-a7)"),
                ]}
            >
                {"With trim"}
            </Text>
        </Flex>
    }
}
