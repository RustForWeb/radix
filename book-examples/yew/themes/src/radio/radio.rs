use radix_yew_themes::{Flex, FlexAlign, FlexChildProps, FlexDirection, Radio, Text, TextAs};
use yew::prelude::*;

#[function_component]
pub fn RadioExample() -> Html {
    html! {
        <Flex align={FlexAlign::Start} direction={FlexDirection::Column} gap=1>
            <Flex
                gap=2

                as_child={|FlexChildProps { class, style, .. }| html! {
                    <Text r#as={TextAs::Label} size=2 class={class} style={style}>
                        <Radio name="example" value="1" default_checked=true />
                        {"Default"}
                    </Text>
                }}
            />

            <Flex
                gap=2

                as_child={|FlexChildProps { class, style, .. }| html! {
                    <Text r#as={TextAs::Label} size=2 class={class} style={style}>
                        <Radio name="example" value="2" />
                        {"Comfortable"}
                    </Text>
                }}
            />

            <Flex
                gap=2

                as_child={|FlexChildProps { class, style, .. }| html! {
                    <Text r#as={TextAs::Label} size=2 class={class} style={style}>
                        <Radio name="example" value="3" />
                        {"Compact"}
                    </Text>
                }}
            />
        </Flex>
    }
}
