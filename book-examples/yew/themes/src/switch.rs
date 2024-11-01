#[allow(clippy::module_inception)]
mod switch;
mod switch_size;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum SwitchRoute {
    #[at("/")]
    Root,
    #[at("/size")]
    Size,
}

pub fn render(route: SwitchRoute) -> Html {
    match route {
        SwitchRoute::Root => html! { <switch::SwitchExample /> },
        SwitchRoute::Size => html! { <switch_size::SwitchSizeExample />},
    }
}
