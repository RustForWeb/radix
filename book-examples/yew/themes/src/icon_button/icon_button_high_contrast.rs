use radix_yew_icons::MagnifyingGlassIcon;
use radix_yew_themes::{AccentColor, Flex, FlexDirection, IconButton, IconButtonVariant};
use yew::prelude::*;

#[function_component]
pub fn IconButtonHighContrastExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Flex gap=3>
                <IconButton color={AccentColor::Gray} variant={IconButtonVariant::Classic}>
                    <MagnifyingGlassIcon width="18" height="18" />
                </IconButton>
                <IconButton color={AccentColor::Gray} variant={IconButtonVariant::Solid}>
                    <MagnifyingGlassIcon width="18" height="18" />
                </IconButton>
                <IconButton color={AccentColor::Gray} variant={IconButtonVariant::Soft}>
                    <MagnifyingGlassIcon width="18" height="18" />
                </IconButton>
                <IconButton color={AccentColor::Gray} variant={IconButtonVariant::Surface}>
                    <MagnifyingGlassIcon width="18" height="18" />
                </IconButton>
                <IconButton color={AccentColor::Gray} variant={IconButtonVariant::Outline}>
                    <MagnifyingGlassIcon width="18" height="18" />
                </IconButton>
            </Flex>
            <Flex gap=3>
                <IconButton color={AccentColor::Gray} variant={IconButtonVariant::Classic} high_contrast=true>
                    <MagnifyingGlassIcon width="18" height="18" />
                </IconButton>
                <IconButton color={AccentColor::Gray} variant={IconButtonVariant::Solid} high_contrast=true>
                    <MagnifyingGlassIcon width="18" height="18" />
                </IconButton>
                <IconButton color={AccentColor::Gray} variant={IconButtonVariant::Soft} high_contrast=true>
                    <MagnifyingGlassIcon width="18" height="18" />
                </IconButton>
                <IconButton color={AccentColor::Gray} variant={IconButtonVariant::Surface} high_contrast=true>
                    <MagnifyingGlassIcon width="18" height="18" />
                </IconButton>
                <IconButton color={AccentColor::Gray} variant={IconButtonVariant::Outline} high_contrast=true>
                    <MagnifyingGlassIcon width="18" height="18" />
                </IconButton>
            </Flex>
        </Flex>
    }
}
