#[allow(clippy::module_inception)]
mod inset;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum InsetRoute {
    #[at("/")]
    Root,
}

pub fn render(route: InsetRoute) -> Html {
    match route {
        InsetRoute::Root => html! { <inset::InsetExample /> },
    }
}
