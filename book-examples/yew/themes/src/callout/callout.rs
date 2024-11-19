use radix_yew_icons::InfoCircledIcon;
use radix_yew_themes::{Callout, CalloutIcon, CalloutText};
use yew::prelude::*;

#[function_component]
pub fn CalloutExample() -> Html {
    html! {
        <Callout>
            <CalloutIcon>
                <InfoCircledIcon />
            </CalloutIcon>
            <CalloutText>
                {"You will need admin privileges to install and access this application."}
            </CalloutText>
        </Callout>
    }
}
