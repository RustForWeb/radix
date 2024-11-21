#[allow(clippy::module_inception)]
mod table;
mod table_size;
mod table_with_a_backplate;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum TableRoute {
    #[at("/")]
    Root,
    #[at("/size")]
    Size,
    #[at("/with-a-backplate")]
    WithABackplate,
}

pub fn render(route: TableRoute) -> Html {
    match route {
        TableRoute::Root => html! { <table::TableExample /> },
        TableRoute::Size => html! { <table_size::TableSizeExample /> },
        TableRoute::WithABackplate => {
            html! { <table_with_a_backplate::TableWithABackplateExample /> }
        }
    }
}
