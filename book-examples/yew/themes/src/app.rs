use radix_yew_themes::{AccentColor, Theme};
use yew::{prelude::*, virtual_dom::VNode};

#[function_component]
pub fn App() -> Html {
    let mut children: Vec<VNode> = vec![];

    #[cfg(feature = "box")]
    {
        use crate::r#box::r#box::BoxExample;
        children.push(html! {
            <BoxExample />
        });
    }
    #[cfg(feature = "button")]
    {
        use crate::button::button::ButtonExample;
        children.push(html! {
            <ButtonExample />
        });
    }
    #[cfg(feature = "button-color")]
    {
        use crate::button::button_color::ButtonColorExample;
        children.push(html! {
            <ButtonColorExample />
        });
    }
    #[cfg(feature = "button-high-contrast")]
    {
        use crate::button::button_high_contrast::ButtonHighContrastExample;
        children.push(html! {
            <ButtonHighContrastExample />
        });
    }
    #[cfg(feature = "button-loading")]
    {
        use crate::button::button_loading::ButtonLoadingExample;
        children.push(html! {
            <ButtonLoadingExample />
        });
    }
    #[cfg(feature = "button-loading-spinner")]
    {
        use crate::button::button_loading_spinner::ButtonLoadingSpinnerExample;
        children.push(html! {
            <ButtonLoadingSpinnerExample />
        });
    }
    #[cfg(feature = "button-radius")]
    {
        use crate::button::button_radius::ButtonRadiusExample;
        children.push(html! {
            <ButtonRadiusExample />
        });
    }
    #[cfg(feature = "button-size")]
    {
        use crate::button::button_size::ButtonSizeExample;
        children.push(html! {
            <ButtonSizeExample />
        });
    }
    #[cfg(feature = "button-variant")]
    {
        use crate::button::button_variant::ButtonVariantExample;
        children.push(html! {
            <ButtonVariantExample />
        });
    }
    #[cfg(feature = "button-variant-ghost")]
    {
        use crate::button::button_variant_ghost::ButtonVariantGhostExample;
        children.push(html! {
            <ButtonVariantGhostExample />
        });
    }
    #[cfg(feature = "button-with-icons")]
    {
        use crate::button::button_with_icons::ButtonWithIconsExample;
        children.push(html! {
            <ButtonWithIconsExample />
        });
    }
    #[cfg(feature = "container")]
    {
        use crate::container::container::ContainerExample;
        children.push(html! {
            <ContainerExample />
        });
    }
    #[cfg(feature = "em")]
    {
        use crate::em::em::EmExample;
        children.push(html! {
            <EmExample />
        });
    }
    #[cfg(feature = "em-truncate")]
    {
        use crate::em::em_truncate::EmTruncateExample;
        children.push(html! {
            <EmTruncateExample />
        });
    }
    #[cfg(feature = "flex")]
    {
        use crate::flex::flex::FlexExample;
        children.push(html! {
            <FlexExample />
        });
    }
    #[cfg(feature = "grid")]
    {
        use crate::grid::grid::GridExample;
        children.push(html! {
            <GridExample />
        });
    }
    #[cfg(feature = "grid-responsive")]
    {
        use crate::grid::grid_responsive::GridResponsiveExample;
        children.push(html! {
            <GridResponsiveExample />
        });
    }
    #[cfg(feature = "heading")]
    {
        use crate::heading::heading::HeadingExample;
        children.push(html! {
            <HeadingExample />
        });
    }
    #[cfg(feature = "heading-align")]
    {
        use crate::heading::heading_align::HeadingAlignExample;
        children.push(html! {
            <HeadingAlignExample />
        });
    }
    #[cfg(feature = "heading-as")]
    {
        use crate::heading::heading_as::HeadingAsExample;
        children.push(html! {
            <HeadingAsExample />
        });
    }
    #[cfg(feature = "heading-color")]
    {
        use crate::heading::heading_color::HeadingColorExample;
        children.push(html! {
            <HeadingColorExample />
        });
    }
    #[cfg(feature = "heading-high-contrast")]
    {
        use crate::heading::heading_high_contrast::HeadingHighContrastExample;
        children.push(html! {
            <HeadingHighContrastExample />
        });
    }
    #[cfg(feature = "heading-size")]
    {
        use crate::heading::heading_size::HeadingSizeExample;
        children.push(html! {
            <HeadingSizeExample />
        });
    }
    #[cfg(feature = "heading-trim")]
    {
        use crate::heading::heading_trim::HeadingTrimExample;
        children.push(html! {
            <HeadingTrimExample />
        });
    }
    #[cfg(feature = "heading-trim-box")]
    {
        use crate::heading::heading_trim_box::HeadingTrimBoxExample;
        children.push(html! {
            <HeadingTrimBoxExample />
        });
    }
    #[cfg(feature = "heading-truncate")]
    {
        use crate::heading::heading_truncate::HeadingTruncateExample;
        children.push(html! {
            <HeadingTruncateExample />
        });
    }
    #[cfg(feature = "heading-weight")]
    {
        use crate::heading::heading_weight::HeadingWeightExample;
        children.push(html! {
            <HeadingWeightExample />
        });
    }
    #[cfg(feature = "heading-wrap")]
    {
        use crate::heading::heading_wrap::HeadingWrapExample;
        children.push(html! {
            <HeadingWrapExample />
        });
    }
    #[cfg(feature = "kbd")]
    {
        use crate::kbd::kbd::KbdExample;
        children.push(html! {
            <KbdExample />
        });
    }
    #[cfg(feature = "kbd-size")]
    {
        use crate::kbd::kbd_size::KbdSizeExample;
        children.push(html! {
            <KbdSizeExample />
        });
    }
    #[cfg(feature = "quote")]
    {
        use crate::quote::quote::QuoteExample;
        children.push(html! {
            <QuoteExample />
        });
    }
    #[cfg(feature = "quote-truncate")]
    {
        use crate::quote::quote_truncate::QuoteTruncateExample;
        children.push(html! {
            <QuoteTruncateExample />
        });
    }
    #[cfg(feature = "section")]
    {
        use crate::section::section::SectionExample;
        children.push(html! {
            <SectionExample />
        });
    }
    #[cfg(feature = "select")]
    {
        use crate::select::select::SelectExample;
        children.push(html! {
            <div style="min-height: 300px;">
                <SelectExample />
            </div>
        });
    }
    #[cfg(feature = "strong")]
    {
        use crate::strong::strong::StrongExample;
        children.push(html! {
            <StrongExample />
        });
    }
    #[cfg(feature = "strong-truncate")]
    {
        use crate::strong::strong_truncate::StrongTruncateExample;
        children.push(html! {
            <StrongTruncateExample />
        });
    }
    #[cfg(feature = "switch")]
    {
        use crate::switch::switch::SwitchExample;
        children.push(html! {
            <SwitchExample />
        });
    }
    #[cfg(feature = "switch-size")]
    {
        use crate::switch::switch_size::SwitchSizeExample;
        children.push(html! {
            <SwitchSizeExample />
        });
    }
    #[cfg(feature = "text")]
    {
        use crate::text::text::TextExample;
        children.push(html! {
            <TextExample />
        });
    }
    #[cfg(feature = "text-align")]
    {
        use crate::text::text_align::TextAlignExample;
        children.push(html! {
            <TextAlignExample />
        });
    }
    #[cfg(feature = "text-as")]
    {
        use crate::text::text_as::TextAsExample;
        children.push(html! {
            <TextAsExample />
        });
    }
    #[cfg(feature = "text-color")]
    {
        use crate::text::text_color::TextColorExample;
        children.push(html! {
            <TextColorExample />
        });
    }
    #[cfg(feature = "text-form-controls")]
    {
        use crate::text::text_form_controls::TextFormControlsExample;
        children.push(html! {
            <TextFormControlsExample />
        });
    }
    #[cfg(feature = "text-formatting")]
    {
        use crate::text::text_formatting::TextFormattingExample;
        children.push(html! {
            <TextFormattingExample />
        });
    }
    #[cfg(feature = "text-high-contrast")]
    {
        use crate::text::text_high_contrast::TextHighContrastExample;
        children.push(html! {
            <TextHighContrastExample />
        });
    }
    #[cfg(feature = "text-size")]
    {
        use crate::text::text_size::TextSizeExample;
        children.push(html! {
            <TextSizeExample />
        });
    }
    #[cfg(feature = "text-size-content")]
    {
        use crate::text::text_size_content::TextSizeContentExample;
        children.push(html! {
            <TextSizeContentExample />
        });
    }
    #[cfg(feature = "text-size-labels")]
    {
        use crate::text::text_size_labels::TextSizeLabelsExample;
        children.push(html! {
            <TextSizeLabelsExample />
        });
    }
    #[cfg(feature = "text-trim")]
    {
        use crate::text::text_trim::TextTrimExample;
        children.push(html! {
            <TextTrimExample />
        });
    }
    #[cfg(feature = "text-trim-box")]
    {
        use crate::text::text_trim_box::TextTrimBoxExample;
        children.push(html! {
            <TextTrimBoxExample />
        });
    }
    #[cfg(feature = "text-truncate")]
    {
        use crate::text::text_truncate::TextTruncateExample;
        children.push(html! {
            <TextTruncateExample />
        });
    }
    #[cfg(feature = "text-weight")]
    {
        use crate::text::text_weight::TextWeightExample;
        children.push(html! {
            <TextWeightExample />
        });
    }
    #[cfg(feature = "text-wrap")]
    {
        use crate::text::text_wrap::TextWrapExample;
        children.push(html! {
            <TextWrapExample />
        });
    }

    html! {
        <Theme accent_color={AccentColor::Indigo}>
            <div style="padding: var(--space-4); line-height: 1;">
                {children}
            </div>
        </Theme>
    }
}
