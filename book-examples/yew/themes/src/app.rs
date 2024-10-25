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

    html! {
        <Theme accent_color={AccentColor::Indigo}>
            <div style="padding: var(--space-4); line-height: 1;">
                {children}
            </div>
        </Theme>
    }
}
