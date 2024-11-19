use radix_yew_icons::InfoCircledIcon;
use radix_yew_themes::{Callout, CalloutIcon, CalloutText, Flex, FlexAlign, FlexDirection};
use yew::prelude::*;

#[function_component]
pub fn CalloutSizeExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3 align={FlexAlign::Start}>
            <Callout size=1>
                <CalloutIcon>
                    <InfoCircledIcon />
                </CalloutIcon>
                <CalloutText>
                    {"You will need admin privileges to install and access this application."}
                </CalloutText>
            </Callout>

            <Callout size=2>
                <CalloutIcon>
                    <InfoCircledIcon />
                </CalloutIcon>
                <CalloutText>
                    {"You will need admin privileges to install and access this application."}
                </CalloutText>
            </Callout>

            <Callout size=3>
                <CalloutIcon>
                    <InfoCircledIcon />
                </CalloutIcon>
                <CalloutText>
                    {"You will need admin privileges to install and access this application."}
                </CalloutText>
            </Callout>
        </Flex>
    }
}
