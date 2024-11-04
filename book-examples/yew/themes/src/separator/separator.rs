use radix_yew_themes::{Flex, FlexAlign, Separator, SeparatorOrientation, Text};
use yew::prelude::*;

#[function_component]
pub fn SeparatorExample() -> Html {
    html! {
        <Text size=2>
            {"Tools for building high-quality, accessible UI."}
            <Separator my=3 size=4 />
            <Flex gap=3 align={FlexAlign::Center}>
                {"Themes"}
                <Separator orientation={SeparatorOrientation::Vertical} />
                {"Primitives"}
                <Separator orientation={SeparatorOrientation::Vertical} />
                {"Icons"}
                <Separator orientation={SeparatorOrientation::Vertical} />
                {"Colors"}
            </Flex>
        </Text>
    }
}
