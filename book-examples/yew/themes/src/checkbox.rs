#[allow(clippy::module_inception)]
mod checkbox;
mod checkbox_alignment;
mod checkbox_alignment_multi_line;
mod checkbox_color;
mod checkbox_disabled;
mod checkbox_high_contrast;
mod checkbox_indeterminate;
mod checkbox_size;
mod checkbox_variant;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum CheckboxRoute {
    #[at("/")]
    Root,
    #[at("/alignment")]
    Alignment,
    #[at("/alignment-multi-line")]
    AlignmentMultiLine,
    #[at("/color")]
    Color,
    #[at("/disabled")]
    Disabled,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/indeterminate")]
    Indeterminate,
    #[at("/size")]
    Size,
    #[at("/variant")]
    Variant,
}

pub fn render(route: CheckboxRoute) -> Html {
    match route {
        CheckboxRoute::Root => html! { <checkbox::CheckboxExample />},
        CheckboxRoute::Alignment => {
            html! { <checkbox_alignment::CheckboxAlignmentExample />}
        }
        CheckboxRoute::AlignmentMultiLine => {
            html! { <checkbox_alignment_multi_line::CheckboxAlignmentMultiLineExample />}
        }
        CheckboxRoute::Color => html! { <checkbox_color::CheckboxColorExample />},
        CheckboxRoute::Disabled => html! { <checkbox_disabled::CheckboxDisabledExample />},
        CheckboxRoute::HighContrast => {
            html! { <checkbox_high_contrast::CheckboxHighContrastExample />}
        }
        CheckboxRoute::Indeterminate => {
            html! { <checkbox_indeterminate::CheckboxIndeterminateExample />}
        }
        CheckboxRoute::Size => html! { <checkbox_size::CheckboxSizeExample />},
        CheckboxRoute::Variant => html! { <checkbox_variant::CheckboxVariantExample />},
    }
}
