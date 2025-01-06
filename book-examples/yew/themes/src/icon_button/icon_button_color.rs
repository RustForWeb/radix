use radix_yew_icons::MagnifyingGlassIcon;
use radix_yew_themes::{AccentColor, Flex, IconButton, IconButtonVariant};
use yew::prelude::*;

#[function_component]
pub fn IconButtonColorExample() -> Html {
    html! {
        <Flex gap=3>
            <IconButton color={AccentColor::Indigo} variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
            <IconButton color={AccentColor::Cyan} variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
            <IconButton color={AccentColor::Orange} variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
            <IconButton color={AccentColor::Crimson} variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
        </Flex>
    }
}
