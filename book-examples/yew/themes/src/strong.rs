#[allow(clippy::module_inception)]
mod strong;
mod strong_truncate;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum StrongRoute {
    #[at("/")]
    Root,
    #[at("/truncate")]
    Truncate,
}

pub fn render(route: StrongRoute) -> Html {
    match route {
        StrongRoute::Root => html! { <strong::StrongExample /> },
        StrongRoute::Truncate => html! { <strong_truncate::StrongTruncateExample />},
    }
}
