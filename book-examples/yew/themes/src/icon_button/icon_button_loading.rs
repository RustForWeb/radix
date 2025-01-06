use radix_yew_icons::MagnifyingGlassIcon;
use radix_yew_themes::{Flex, IconButton, IconButtonVariant};
use yew::prelude::*;

#[function_component]
pub fn IconButtonLoadingExample() -> Html {
    html! {
        <Flex gap=3>
            <IconButton loading=true variant={IconButtonVariant::Classic}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
            <IconButton loading=true variant={IconButtonVariant::Solid}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
            <IconButton loading=true variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
            <IconButton loading=true variant={IconButtonVariant::Surface}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
            <IconButton loading=true variant={IconButtonVariant::Outline}>
                <MagnifyingGlassIcon width=18 height=18 />
            </IconButton>
        </Flex>
    }
}
