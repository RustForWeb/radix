use radix_yew_themes::{AccentColor, Checkbox, CheckedState, Flex};
use yew::prelude::*;

#[function_component]
pub fn CheckboxColorExample() -> Html {
    html! {
        <Flex gap=2>
            <Checkbox color={AccentColor::Indigo} default_checked={CheckedState::True} />
            <Checkbox color={AccentColor::Cyan} default_checked={CheckedState::True} />
            <Checkbox color={AccentColor::Orange} default_checked={CheckedState::True} />
            <Checkbox color={AccentColor::Crimson} default_checked={CheckedState::True} />
        </Flex>
    }
}
