use radix_yew_themes::{Flex, FlexAlign, FlexChildProps, FlexDirection, Radio, Text, TextAs};
use yew::prelude::*;

#[function_component]
pub fn RadioAlignmentExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Flex align={FlexAlign::Start} direction={FlexDirection::Column} gap=1>
                <Flex
                    gap=2

                    as_child={|FlexChildProps { class, style, .. }| html! {
                        <Text r#as={TextAs::Label} size=2 class={class} style={style}>
                            <Radio size=1 name="alignment-1" value="1" default_checked=true />
                            {"Default"}
                        </Text>
                    }}
                />
                <Flex
                    gap=2

                    as_child={|FlexChildProps { class, style, .. }| html! {
                        <Text r#as={TextAs::Label} size=2 class={class} style={style}>
                            <Radio size=1 name="alignment-1" value="2" />
                            {"Compact"}
                        </Text>
                    }}
                />
            </Flex>

            <Flex align={FlexAlign::Start} direction={FlexDirection::Column} gap=1>
                <Flex
                    gap=2

                    as_child={|FlexChildProps { class, style, .. }| html! {
                        <Text r#as={TextAs::Label} size=3 class={class} style={style}>
                            <Radio size=2 name="alignment-2" value="1" default_checked=true />
                            {"Default"}
                        </Text>
                    }}
                />
                <Flex
                    gap=2

                    as_child={|FlexChildProps { class, style, .. }| html! {
                        <Text r#as={TextAs::Label} size=3 class={class} style={style}>
                            <Radio size=2 name="alignment-2" value="2" />
                            {"Compact"}
                        </Text>
                    }}
                />
            </Flex>

            <Flex align={FlexAlign::Start} direction={FlexDirection::Column} gap=1>
                <Flex
                    gap=2

                    as_child={|FlexChildProps { class, style, .. }| html! {
                        <Text r#as={TextAs::Label} size=4 class={class} style={style}>
                            <Radio size=3 name="alignment-3" value="1" default_checked=true />
                            {"Default"}
                        </Text>
                    }}
                />
                <Flex
                    gap=2

                    as_child={|FlexChildProps { class, style, .. }| html! {
                        <Text r#as={TextAs::Label} size=4 class={class} style={style}>
                            <Radio size=3 name="alignment-3" value="2" />
                            {"Compact"}
                        </Text>
                    }}
                />
            </Flex>
        </Flex>
    }
}
