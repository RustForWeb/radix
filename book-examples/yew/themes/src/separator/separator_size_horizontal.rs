use radix_yew_themes::{Flex, FlexDirection, Separator, SeparatorOrientation};
use yew::prelude::*;

#[function_component]
pub fn SeparatorSizeHorizontalExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=4>
            <Separator orientation={SeparatorOrientation::Horizontal} size=1 />
            <Separator orientation={SeparatorOrientation::Horizontal} size=2 />
            <Separator orientation={SeparatorOrientation::Horizontal} size=3 />
            <Separator orientation={SeparatorOrientation::Horizontal} size=4 />
        </Flex>
    }
}
