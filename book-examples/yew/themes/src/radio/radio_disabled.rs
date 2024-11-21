use radix_yew_themes::{Flex, FlexAlign, FlexChildProps, FlexDirection, Radio, Text, TextAs};
use yew::prelude::*;

#[function_component]
pub fn RadioDisabledExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Flex align={FlexAlign::Start} direction={FlexDirection::Column} gap=1>
                <Flex
                    gap=2

                    as_child={|FlexChildProps { class, style, .. }| html! {
                        <Text r#as={TextAs::Label} size=2 class={class} style={style}>
                            <Radio name="enabled" value="1" default_checked=true />
                            {"On"}
                        </Text>
                    }}
                />
                <Flex
                    gap=2

                    as_child={|FlexChildProps { class, style, .. }| html! {
                        <Text r#as={TextAs::Label} size=2 class={class} style={style}>
                            <Radio name="enabled" value="2" />
                            {"Off"}
                        </Text>
                    }}
                />
            </Flex>

            <Flex align={FlexAlign::Start} direction={FlexDirection::Column} gap=1>
                <Flex
                    gap=2

                    as_child={|FlexChildProps { class, style, .. }| html! {
                        <Text r#as={TextAs::Label} size=2 class={class} style={style}>
                            <Radio disabled=true name="disabled" value="1" default_checked=true />
                            {"On"}
                        </Text>
                    }}
                />
                <Flex
                    gap=2

                    as_child={|FlexChildProps { class, style, .. }| html! {
                        <Text r#as={TextAs::Label} size=2 class={class} style={style}>
                            <Radio disabled=true name="disabled" value="2" />
                            {"Off"}
                        </Text>
                    }}
                />
            </Flex>
        </Flex>
    }
}
