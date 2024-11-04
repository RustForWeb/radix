use radix_yew_themes::{Flex, FlexAlign, Separator, SeparatorOrientation};
use yew::prelude::*;

#[function_component]
pub fn SeparatorOrientationExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=4>
            <Separator orientation={SeparatorOrientation::Horizontal} />
            <Separator orientation={SeparatorOrientation::Vertical} />
        </Flex>
    }
}
