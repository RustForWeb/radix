#[allow(clippy::module_inception)]
mod text;
mod text_align;
mod text_as;
mod text_color;
mod text_form_controls;
mod text_formatting;
mod text_high_contrast;
mod text_size;
mod text_size_content;
mod text_size_labels;
mod text_trim;
mod text_trim_box;
mod text_truncate;
mod text_weight;
mod text_wrap;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum TextRoute {
    #[at("/")]
    Root,
    #[at("/align")]
    Align,
    #[at("/as")]
    As,
    #[at("/color")]
    Color,
    #[at("/form-controls")]
    FormControls,
    #[at("/formatting")]
    Formatting,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/size")]
    Size,
    #[at("/size-content")]
    SizeContent,
    #[at("/size-labels")]
    SizeLabels,
    #[at("/trim")]
    Trim,
    #[at("/trim-box")]
    TrimBox,
    #[at("/truncate")]
    Truncate,
    #[at("/weight")]
    Weight,
    #[at("/wrap")]
    Wrap,
}

pub fn render(route: TextRoute) -> Html {
    match route {
        TextRoute::Root => html! { <text::TextExample />},
        TextRoute::Align => html! { <text_align::TextAlignExample />},
        TextRoute::As => html! { <text_as::TextAsExample />},
        TextRoute::Color => html! { <text_color::TextColorExample />},
        TextRoute::FormControls => html! { <text_form_controls::TextFormControlsExample />},
        TextRoute::Formatting => html! { <text_formatting::TextFormattingExample />},
        TextRoute::HighContrast => {
            html! { <text_high_contrast::TextHighContrastExample />}
        }
        TextRoute::Size => html! { <text_size::TextSizeExample />},
        TextRoute::SizeContent => html! { <text_size_content::TextSizeContentExample />},
        TextRoute::SizeLabels => html! { <text_size_labels::TextSizeLabelsExample />},
        TextRoute::Trim => html! { <text_trim::TextTrimExample />},
        TextRoute::TrimBox => html! { <text_trim_box::TextTrimBoxExample />},
        TextRoute::Truncate => html! { <text_truncate::TextTruncateExample />},
        TextRoute::Weight => html! { <text_weight::TextWeightExample />},
        TextRoute::Wrap => html! { <text_wrap::TextWrapExample />},
    }
}
