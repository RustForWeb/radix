use radix_yew_icons::MagnifyingGlassIcon;
use radix_yew_themes::{Flex, IconButton, IconButtonVariant};
use yew::prelude::*;

#[function_component]
pub fn IconButtonVariantExample() -> Html {
    html! {
        <Flex gap=3>
            <IconButton variant={IconButtonVariant::Classic}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
            <IconButton variant={IconButtonVariant::Solid}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
            <IconButton variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
            <IconButton variant={IconButtonVariant::Surface}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
            <IconButton variant={IconButtonVariant::Outline}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
        </Flex>
    }
}
