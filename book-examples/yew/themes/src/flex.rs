#[allow(clippy::module_inception)]
mod flex;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum FlexRoute {
    #[at("/")]
    Root,
}

pub fn render(route: FlexRoute) -> Html {
    match route {
        FlexRoute::Root => html! { <flex::FlexExample /> },
    }
}
