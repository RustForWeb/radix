use radix_yew_themes::{Flex, FlexAlign, Radio, RadioVariant};
use yew::prelude::*;

#[function_component]
pub fn RadioVariantExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=4>
            <Flex gap=2>
                <Radio variant={RadioVariant::Surface} name="surface" value="1" default_checked=true />
                <Radio variant={RadioVariant::Surface} name="surface" value="2" />
            </Flex>

            <Flex gap=2>
                <Radio variant={RadioVariant::Classic} name="classic" value="1" default_checked=true />
                <Radio variant={RadioVariant::Classic} name="classic" value="2" />
            </Flex>

            <Flex gap=2>
                <Radio variant={RadioVariant::Soft} name="soft" value="1" default_checked=true />
                <Radio variant={RadioVariant::Soft} name="soft" value="2" />
            </Flex>
        </Flex>
    }
}
