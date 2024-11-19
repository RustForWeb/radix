use radix_yew_icons::InfoCircledIcon;
use radix_yew_themes::{
    Callout, CalloutIcon, CalloutText, CalloutVariant, Flex, FlexDirection, Link,
};
use yew::prelude::*;

#[function_component]
pub fn CalloutVariantExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Callout variant={CalloutVariant::Soft}>
                <CalloutIcon>
                    <InfoCircledIcon />
                </CalloutIcon>
                <CalloutText>
                    {"You will need "}<Link href="#">{"admin privileges"}</Link>{" to install and access
                    this application."}
                </CalloutText>
            </Callout>

            <Callout variant={CalloutVariant::Surface}>
                <CalloutIcon>
                    <InfoCircledIcon />
                </CalloutIcon>
                <CalloutText>
                    {"You will need "}<Link href="#">{"admin privileges"}</Link>{" to install and access
                    this application."}
                </CalloutText>
            </Callout>

            <Callout variant={CalloutVariant::Outline}>
                <CalloutIcon>
                    <InfoCircledIcon />
                </CalloutIcon>
                <CalloutText>
                    {"You will need "}<Link href="#">{"admin privileges"}</Link>{" to install and access
                    this application."}
                </CalloutText>
            </Callout>
        </Flex>
    }
}
