use radix_yew_themes::{AccentColor, Card, CardVariant, Flex, FlexDirection, Text, TextAs, Weight};
use yew::prelude::*;

#[function_component]
pub fn CardVariantExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 max_width="350px">
            <Card variant={CardVariant::Surface}>
                <Text r#as={TextAs::Div} size=2 weight={Weight::Bold}>
                    {"Quick start"}
                </Text>
                <Text r#as={TextAs::Div} color={AccentColor::Gray} size=2>
                    {"Start building your next project in minutes"}
                </Text>
            </Card>

            <Card variant={CardVariant::Classic}>
                <Text r#as={TextAs::Div} size=2 weight={Weight::Bold}>
                    {"Quick start"}
                </Text>
                <Text r#as={TextAs::Div} color={AccentColor::Gray} size=2>
                    {"Start building your next project in minutes"}
                </Text>
            </Card>
        </Flex>
    }
}
