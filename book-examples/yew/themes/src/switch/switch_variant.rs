use radix_yew_themes::{Flex, FlexDirection, Switch, SwitchVariant};
use yew::prelude::*;

#[function_component]
pub fn SwitchVariantExample() -> Html {
    html! {
        <Flex gap=2>
            <Flex direction={FlexDirection::Column} gap=3>
                <Switch variant={SwitchVariant::Surface} />
                <Switch variant={SwitchVariant::Classic} />
                <Switch variant={SwitchVariant::Soft} />
            </Flex>

            <Flex direction={FlexDirection::Column} gap=3>
                <Switch variant={SwitchVariant::Surface} default_checked=true />
                <Switch variant={SwitchVariant::Classic} default_checked=true />
                <Switch variant={SwitchVariant::Soft} default_checked=true />
            </Flex>
        </Flex>
    }
}
