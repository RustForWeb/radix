#[allow(clippy::module_inception)]
mod select;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum SelectRoute {
    #[at("/")]
    Root,
}

pub fn render(route: SelectRoute) -> Html {
    match route {
        SelectRoute::Root => html! { <select::SelectExample /> },
    }
}
