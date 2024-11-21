use radix_yew_themes::{Flex, Skeleton, SkeletonChildProps, Switch};
use yew::prelude::*;

#[function_component]
pub fn SkeletonWithChildrenExample() -> Html {
    html! {
        <Flex gap=4>
            <Skeleton
                loading=true

                as_child={Callback::from(|SkeletonChildProps {
                    aria_hidden,
                    class,
                    inert,
                    style,
                    tabindex,
                    ..
                }| html! {
                    <Switch
                        default_checked=true

                        class={class}
                        style={style}

                        attributes={[
                            ("aria-hidden", aria_hidden),
                            ("inert", inert),
                            ("tabindex", tabindex),
                        ]}
                    />
                })}>
            </Skeleton>

            <Skeleton
                loading=false

                as_child={Callback::from(|SkeletonChildProps {
                    aria_hidden,
                    class,
                    inert,
                    style,
                    tabindex,
                    ..
                }| html! {
                    <Switch
                        default_checked=true

                        class={class}
                        style={style}

                        attributes={[
                            ("aria-hidden", aria_hidden),
                            ("inert", inert),
                            ("tabindex", tabindex),
                        ]}
                    />
                })}>
            </Skeleton>
        </Flex>
    }
}
