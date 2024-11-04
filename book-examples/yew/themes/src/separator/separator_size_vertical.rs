use radix_yew_themes::{Flex, FlexAlign, Separator, SeparatorOrientation};
use yew::prelude::*;

#[function_component]
pub fn SeparatorSizeVerticalExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=4 height="96px">
            <Separator orientation={SeparatorOrientation::Vertical} size=1 />
            <Separator orientation={SeparatorOrientation::Vertical} size=2 />
            <Separator orientation={SeparatorOrientation::Vertical} size=3 />
            <Separator orientation={SeparatorOrientation::Vertical} size=4 />
        </Flex>
    }
}
