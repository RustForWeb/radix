use radix_yew_themes::{AccentColor, Flex, FlexDirection, Switch, Text, TextAs};
use yew::prelude::*;

#[function_component]
pub fn SwitchDisabledExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=2>
            <Text r#as={TextAs::Label} size=2>
                <Flex gap=2>
                    <Switch size=1 />
                    {"Off"}
                </Flex>
            </Text>

            <Text r#as={TextAs::Label} size=2>
                <Flex gap=2>
                    <Switch size=1 default_checked=true />
                    {"On"}
                </Flex>
            </Text>

            <Text r#as={TextAs::Label} size=2 color={AccentColor::Gray}>
                <Flex gap=2>
                    <Switch size=1 disabled=true />
                    {"On"}
                </Flex>
            </Text>

            <Text r#as={TextAs::Label} size=2 color={AccentColor::Gray}>
                <Flex gap=2>
                    <Switch size=1 disabled=true default_checked=true />
                    {"Off"}
                </Flex>
            </Text>
        </Flex>
    }
}
