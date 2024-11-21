use radix_yew_themes::{AccentColor, Flex, FlexAs, Radio};
use yew::prelude::*;

#[function_component]
pub fn RadioColorExample() -> Html {
    html! {
        <Flex r#as={FlexAs::Span} gap=2>
            <Radio color={AccentColor::Indigo} default_checked=true />
            <Radio color={AccentColor::Cyan} default_checked=true />
            <Radio color={AccentColor::Orange} default_checked=true />
            <Radio color={AccentColor::Crimson} default_checked=true />
        </Flex>
    }
}
