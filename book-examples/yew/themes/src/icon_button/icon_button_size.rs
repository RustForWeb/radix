use radix_yew_icons::MagnifyingGlassIcon;
use radix_yew_themes::{Flex, FlexAlign, IconButton, IconButtonVariant};
use yew::prelude::*;

#[function_component]
pub fn IconButtonSizeExample() -> Html {
    html! {
        <Flex align={FlexAlign::Center} gap=3>
            <IconButton size=1 variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width="15" height="15" />
            </IconButton>

            <IconButton size=2 variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width="18" height="18" />
            </IconButton>

            <IconButton size=3 variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width="22" height="22" />
            </IconButton>
        </Flex>
    }
}
