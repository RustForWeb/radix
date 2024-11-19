use radix_yew_icons::InfoCircledIcon;
use radix_yew_themes::{AccentColor, Callout, CalloutIcon, CalloutText, Flex, FlexDirection, Link};
use yew::prelude::*;

#[function_component]
pub fn CalloutColorExample() -> Html {
    html! {
        <Flex direction={FlexDirection::Column} gap=3>
            <Callout color={AccentColor::Blue}>
                <CalloutIcon>
                    <InfoCircledIcon />
                </CalloutIcon>
                <CalloutText>
                    {"You will need "}<Link href="#">{"admin privileges"}</Link>{" to install and access
                    this application."}
                </CalloutText>
            </Callout>

            <Callout color={AccentColor::Green}>
                <CalloutIcon>
                    <InfoCircledIcon />
                </CalloutIcon>
                <CalloutText>
                    {"You will need "}<Link href="#">{"admin privileges"}</Link>{" to install and access
                    this application."}
                </CalloutText>
            </Callout>

            <Callout color={AccentColor::Red}>
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
