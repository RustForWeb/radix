#[allow(clippy::module_inception)]
mod section;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum SectionRoute {
    #[at("/")]
    Root,
}

pub fn render(route: SectionRoute) -> Html {
    match route {
        SectionRoute::Root => html! { <section::SectionExample /> },
    }
}
