use radix_yew_themes::{AccentColor, Flex, FlexDirection, Grid, GridAlign, Text, Weight};
use yew::prelude::*;

#[function_component]
pub fn TextLabelsExample() -> Html {
    html! {
        <Grid align={GridAlign::Center} columns=2 gap=5 p=3>
            <Flex direction={FlexDirection::Column}>
                <Text size=3 weight={Weight::Bold}>
                    {"Get started"}
                </Text>
                <Text color={AccentColor::Gray} size=2>
                    {"Start your next project in minutes"}
                </Text>
            </Flex>

            <Flex direction={FlexDirection::Column}>
                <Text size=2 weight={Weight::Bold}>
                    {"Get started"}
                </Text>
                <Text color={AccentColor::Gray} size=2>
                    {"Start your next project in minutes"}
                </Text>
            </Flex>

            <Flex direction={FlexDirection::Column}>
                <Text size=2 weight={Weight::Bold}>
                    {"Get started"}
                </Text>
                <Text color={AccentColor::Gray} size=1>
                    {"Start your next project in minutes"}
                </Text>
            </Flex>

            <Flex direction={FlexDirection::Column}>
                <Text size=1 weight={Weight::Bold}>
                    {"Get started"}
                </Text>
                <Text color={AccentColor::Gray} size=1>
                    {"Start your next project in minutes"}
                </Text>
            </Flex>
        </Grid>
    }
}
