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
    #[cfg(feature = "section")]
    {
        use crate::section::section::SectionExample;
        children.push(html! {
            <SectionExample />
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
