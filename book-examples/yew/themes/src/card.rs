#[allow(clippy::module_inception)]
mod card;
mod card_as_another_element;
mod card_size;
mod card_variant;
mod card_with_inset_content;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum CardRoute {
    #[at("/")]
    Root,
    #[at("/as-another-element")]
    AsAnotherElement,
    #[at("/size")]
    Size,
    #[at("/variant")]
    Variant,
    #[at("/with-inset-content")]
    WithInsetContent,
}

pub fn render(route: CardRoute) -> Html {
    match route {
        CardRoute::Root => html! { <card::CardExample />},
        CardRoute::AsAnotherElement => {
            html! { <card_as_another_element::CardAsAnotherElementExample />}
        }
        CardRoute::Size => html! { <card_size::CardSizeExample />},
        CardRoute::Variant => html! { <card_variant::CardVariantExample />},
        CardRoute::WithInsetContent => {
            html! { <card_with_inset_content::CardWithInsetContentExample />}
        }
    }
}
