use radix_yew_icons::InfoCircledIcon;
use radix_yew_themes::{
    AccentColor, Callout, CalloutIcon, CalloutText, CalloutVariant, Flex, FlexDirection,
};
use yew::prelude::*;

#[function_component]
pub fn CalloutHighContrastExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Callout color={AccentColor::Gray} variant={CalloutVariant::Soft}>
                <CalloutIcon>
                    <InfoCircledIcon />
                </CalloutIcon>
                <CalloutText>
                    {"An update to Radix Themes is available. See what's new in version 3.2.0."}
                </CalloutText>
            </Callout>

            <Callout color={AccentColor::Gray} variant={CalloutVariant::Soft} high_contrast=true>
                <CalloutIcon>
                    <InfoCircledIcon />
                </CalloutIcon>
                <CalloutText>
                    {"An update to Radix Themes is available. See what's new in version 3.2.0."}
                </CalloutText>
            </Callout>
        </Flex>
    }
}
