#[allow(clippy::module_inception)]
mod grid;
mod grid_responsive;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum GridRoute {
    #[at("/")]
    Root,
    #[at("/responsive")]
    Responsive,
}

pub fn render(route: GridRoute) -> Html {
    match route {
        GridRoute::Root => html! { <grid::GridExample/> },
        GridRoute::Responsive => html! { <grid_responsive::GridResponsiveExample /> },
    }
}
