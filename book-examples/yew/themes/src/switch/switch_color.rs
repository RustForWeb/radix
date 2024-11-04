use radix_yew_themes::{AccentColor, Flex, Switch};
use yew::prelude::*;

#[function_component]
pub fn SwitchColorExample() -> Html {
    html! {
        <Flex gap=2>
            <Switch color={AccentColor::Indigo} default_checked=true />
            <Switch color={AccentColor::Cyan} default_checked=true />
            <Switch color={AccentColor::Orange} default_checked=true />
            <Switch color={AccentColor::Crimson} default_checked=true />
        </Flex>
    }
}
