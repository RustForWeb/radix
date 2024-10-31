use radix_yew_themes::{Avatar, Flex};
use yew::prelude::*;

#[function_component]
pub fn AvatarExample() -> Html {
    html! {
        <Flex gap=2>
            <Avatar
                src="https://images.unsplash.com/photo-1502823403499-6ccfcf4fb453?&w=256&h=256&q=70&crop=focalpoint&fp-x=0.5&fp-y=0.3&fp-z=1&fit=crop"
                fallback="A"
            />
            <Avatar fallback="A" />
        </Flex>
    }
}
