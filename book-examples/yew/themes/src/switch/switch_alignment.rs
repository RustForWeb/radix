use radix_yew_themes::{Flex, FlexDirection, Switch, Text, TextAs};
use yew::prelude::*;

#[function_component]
pub fn SwitchAlignmentExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Text r#as={TextAs::Label} size=2>
                <Flex gap=2>
                    <Switch size=1 default_checked=true />{" Sync settings"}
                </Flex>
            </Text>

            <Text r#as={TextAs::Label} size=3>
                <Flex gap=2>
                    <Switch size=2 default_checked=true />{" Sync settings"}
                </Flex>
            </Text>

            <Text r#as={TextAs::Label} size=4>
                <Flex gap=2>
                    <Switch size=3 default_checked=true />{" Sync settings"}
                </Flex>
            </Text>
        </Flex>
    }
}
