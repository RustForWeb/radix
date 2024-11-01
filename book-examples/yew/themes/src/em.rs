#[allow(clippy::module_inception)]
mod em;
mod em_truncate;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum EmRoute {
    #[at("/")]
    Root,
    #[at("/truncate")]
    Truncate,
}

pub fn render(route: EmRoute) -> Html {
    match route {
        EmRoute::Root => html! { <em::EmExample /> },
        EmRoute::Truncate => html! { <em_truncate::EmTruncateExample />},
    }
}
