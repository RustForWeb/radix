use radix_yew_themes::{Flex, FlexDirection, Heading, LeadingTrim};
use yew::prelude::*;

#[function_component]
pub fn HeadingTrimExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Heading
                trim={LeadingTrim::Normal}
                style={[
                    ("background", "var(--gray-a2)"),
                    ("border-top", "1px dashed var(--gray-a7)"),
                    ("border-bottom", "1px dashed var(--gray-a7)"),
                ]}
            >
                {"Without trim"}
            </Heading>
            <Heading
                trim={LeadingTrim::Both}
                style={[
                    ("background", "var(--gray-a2)"),
                    ("border-top", "1px dashed var(--gray-a7)"),
                    ("border-bottom", "1px dashed var(--gray-a7)"),
                ]}
            >
                {"With trim"}
            </Heading>
        </Flex>
    }
}
