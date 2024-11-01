#[allow(clippy::module_inception)]
mod kbd;
mod kbd_size;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum KbdRoute {
    #[at("/")]
    Root,
    #[at("/size")]
    Size,
}

pub fn render(route: KbdRoute) -> Html {
    match route {
        KbdRoute::Root => html! { <kbd::KbdExample /> },
        KbdRoute::Size => html! { <kbd_size::KbdSizeExample />},
    }
}
