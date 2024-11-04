use radix_yew_themes::{AccentColor, Flex, FlexDirection, Separator};
use yew::prelude::*;

#[function_component]
pub fn SeparatorColorExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Separator color={AccentColor::Indigo} size=4 />
            <Separator color={AccentColor::Cyan} size=4 />
            <Separator color={AccentColor::Orange} size=4 />
            <Separator color={AccentColor::Crimson} size=4 />
        </Flex>
    }
}
