use radix_yew_themes::{
    AccentColor, Avatar, Box, Card, Flex, FlexAlign, FlexDirection, Radius, Text, TextAs, Weight,
};
use yew::prelude::*;

#[function_component]
pub fn CardSizeExample() -> Html {
    html! {
        <Flex gap=3 direction={FlexDirection::Column}>
            <Box width="350px">
                <Card size=1>
                    <Flex gap=3 align={FlexAlign::Center}>
                        <Avatar size=3 radius={Radius::Full} fallback="T" color={AccentColor::Indigo} />
                        <Box>
                            <Text r#as={TextAs::Div} size=2 weight={Weight::Bold}>
                                {"Teodros Girmay"}
                            </Text>
                            <Text r#as={TextAs::Div} size=2 color={AccentColor::Gray}>
                                {"Engineering"}
                            </Text>
                        </Box>
                    </Flex>
                </Card>
            </Box>

            <Box width="400px">
                <Card size=2>
                    <Flex gap=4 align={FlexAlign::Center}>
                        <Avatar size=4 radius={Radius::Full} fallback="T" color={AccentColor::Indigo} />
                        <Box>
                            <Text r#as={TextAs::Div} weight={Weight::Bold}>
                                {"Teodros Girmay"}
                            </Text>
                            <Text r#as={TextAs::Div} color={AccentColor::Gray}>
                                {"Engineering"}
                            </Text>
                        </Box>
                    </Flex>
                </Card>
            </Box>

            <Box width="500px">
                <Card size=3>
                    <Flex gap=4 align={FlexAlign::Center}>
                        <Avatar size=5 radius={Radius::Full} fallback="T" color={AccentColor::Indigo} />
                        <Box>
                            <Text r#as={TextAs::Div} size=4 weight={Weight::Bold}>
                                {"Teodros Girmay"}
                            </Text>
                            <Text r#as={TextAs::Div} size=4 color={AccentColor::Gray}>
                                {"Engineering"}
                            </Text>
                        </Box>
                    </Flex>
                </Card>
            </Box>
        </Flex>
    }
}
