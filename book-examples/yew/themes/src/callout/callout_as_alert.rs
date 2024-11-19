use radix_yew_icons::ExclamationTriangleIcon;
use radix_yew_themes::{AccentColor, Callout, CalloutIcon, CalloutText};
use yew::prelude::*;

#[function_component]
pub fn CalloutAsAlertExample() -> Html {
    html! {
        <Callout color={AccentColor::Red} role="alert">
            <CalloutIcon>
                <ExclamationTriangleIcon />
            </CalloutIcon>
            <CalloutText>
                {"Access denied. Please contact the network administrator to view this page."}
            </CalloutText>
        </Callout>
    }
}
