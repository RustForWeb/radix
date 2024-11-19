use radix_yew_themes::{
    AccentColor, Avatar, Box, Card, Flex, FlexAlign, Radius, Text, TextAs, Weight,
};
use yew::prelude::*;

#[function_component]
pub fn CardExample() -> Html {
    html! {
        <Box max_width="240px">
            <Card>
                <Flex gap=3 align={FlexAlign::Center}>
                    <Avatar
                        size=3
                        src="https://images.unsplash.com/photo-1607346256330-dee7af15f7c5?&w=64&h=64&dpr=2&q=70&crop=focalpoint&fp-x=0.67&fp-y=0.5&fp-z=1.4&fit=crop"
                        radius={Radius::Full}
                        fallback="T"
                    />
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
    }
}
