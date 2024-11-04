#[allow(clippy::module_inception)]
mod spinner;
mod spinner_size;
mod spinner_with_buttons;
mod spinner_with_buttons_disabled;
mod spinner_with_children;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum SpinnerRoute {
    #[at("/")]
    Root,
    #[at("/size")]
    Size,
    #[at("/with-buttons")]
    WithButtons,
    #[at("/with-buttons-disabled")]
    WithButtonsDisabled,
    #[at("/with-children")]
    WithChildren,
}

pub fn render(route: SpinnerRoute) -> Html {
    match route {
        SpinnerRoute::Root => html! { <spinner::SpinnerExample/> },
        SpinnerRoute::Size => html! { <spinner_size::SpinnerSizeExample /> },
        SpinnerRoute::WithButtons => html! { <spinner_with_buttons::SpinnerWithButtonsExample /> },
        SpinnerRoute::WithButtonsDisabled => {
            html! { <spinner_with_buttons_disabled::SpinnerWithButtonsDisabledExample /> }
        }
        SpinnerRoute::WithChildren => {
            html! { <spinner_with_children::SpinnerWithChildrenExample /> }
        }
    }
}
