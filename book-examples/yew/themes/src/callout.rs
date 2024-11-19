#[allow(clippy::module_inception)]
mod callout;
mod callout_as_alert;
mod callout_color;
mod callout_high_contrast;
mod callout_size;
mod callout_variant;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum CalloutRoute {
    #[at("/")]
    Root,
    #[at("/as-alert")]
    AsAlert,
    #[at("/color")]
    Color,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/size")]
    Size,
    #[at("/variant")]
    Variant,
}

pub fn render(route: CalloutRoute) -> Html {
    match route {
        CalloutRoute::Root => html! { <callout::CalloutExample />},
        CalloutRoute::AsAlert => html! { <callout_as_alert::CalloutAsAlertExample />},
        CalloutRoute::Color => html! { <callout_color::CalloutColorExample />},
        CalloutRoute::HighContrast => {
            html! { <callout_high_contrast::CalloutHighContrastExample />}
        }
        CalloutRoute::Size => html! { <callout_size::CalloutSizeExample />},
        CalloutRoute::Variant => html! { <callout_variant::CalloutVariantExample />},
    }
}
