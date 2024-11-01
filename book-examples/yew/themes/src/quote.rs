#[allow(clippy::module_inception)]
mod quote;
mod quote_truncate;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum QuoteRoute {
    #[at("/")]
    Root,
    #[at("/truncate")]
    Truncate,
}

pub fn render(route: QuoteRoute) -> Html {
    match route {
        QuoteRoute::Root => html! { <quote::QuoteExample /> },
        QuoteRoute::Truncate => html! { <quote_truncate::QuoteTruncateExample />},
    }
}
