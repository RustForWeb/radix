#[allow(clippy::module_inception)]
mod code;
mod code_color;
mod code_high_contrast;
mod code_size;
mod code_truncate;
mod code_variant;
mod code_weight;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum CodeRoute {
    #[at("/")]
    Root,
    #[at("/color")]
    Color,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/size")]
    Size,
    #[at("/truncate")]
    Truncate,
    #[at("/variant")]
    Variant,
    #[at("/weight")]
    Weight,
}

pub fn render(route: CodeRoute) -> Html {
    match route {
        CodeRoute::Root => html! { <code::CodeExample />},
        CodeRoute::Color => html! { <code_color::CodeColorExample />},
        CodeRoute::HighContrast => {
            html! { <code_high_contrast::CodeHighContrastExample />}
        }
        CodeRoute::Size => html! { <code_size::CodeSizeExample />},
        CodeRoute::Truncate => html! { <code_truncate::CodeTruncateExample />},
        CodeRoute::Variant => html! { <code_variant::CodeVariantExample />},
        CodeRoute::Weight => html! { <code_weight::CodeWeightExample />},
    }
}
