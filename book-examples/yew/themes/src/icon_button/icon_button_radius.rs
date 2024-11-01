use radix_yew_icons::MagnifyingGlassIcon;
use radix_yew_themes::{Flex, IconButton, IconButtonVariant, Radius};
use yew::prelude::*;

#[function_component]
pub fn IconButtonRadiusExample() -> Html {
    html! {
        <Flex gap=3>
            <IconButton radius={Radius::None} variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width="18" height="18" />
            </IconButton>
            <IconButton radius={Radius::Small} variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width="18" height="18" />
            </IconButton>
            <IconButton radius={Radius::Medium} variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width="18" height="18" />
            </IconButton>
            <IconButton radius={Radius::Large} variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width="18" height="18" />
            </IconButton>
            <IconButton radius={Radius::Full} variant={IconButtonVariant::Soft}>
                <MagnifyingGlassIcon width="18" height="18" />
            </IconButton>
        </Flex>
    }
}
