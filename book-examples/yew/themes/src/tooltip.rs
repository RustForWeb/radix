#[allow(clippy::module_inception)]
mod tooltip;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum TooltipRoute {
    #[at("/")]
    Root,
}

pub fn render(route: TooltipRoute) -> Html {
    match route {
        TooltipRoute::Root => html! { <tooltip::TooltipExample /> },
    }
}
