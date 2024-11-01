#[allow(clippy::module_inception)]
mod container;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum ContainerRoute {
    #[at("/")]
    Root,
}

pub fn render(route: ContainerRoute) -> Html {
    match route {
        ContainerRoute::Root => html! { <container::ContainerExample /> },
    }
}
