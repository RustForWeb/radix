#[allow(clippy::module_inception)]
mod r#box;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum BoxRoute {
    #[at("/")]
    Root,
}

pub fn render(route: BoxRoute) -> Html {
    match route {
        BoxRoute::Root => html! { <r#box::BoxExample /> },
    }
}
