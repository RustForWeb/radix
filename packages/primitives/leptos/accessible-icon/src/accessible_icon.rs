use leptos::{
    attr::{
        aria_hidden,
        custom::{custom_attribute, CustomAttr},
        AriaHidden, Attr, NextAttribute,
    },
    prelude::*,
};
use radix_leptos_visually_hidden::VisuallyHidden;

pub type AccessibleIconAttrs = (
    Attr<AriaHidden, &'static str>,
    CustomAttr<&'static str, &'static str>,
);

pub fn use_accessible_icon() -> AccessibleIconAttrs {
    aria_hidden("true").add_any_attr(custom_attribute("focusable", "false"))
}

#[component]
pub fn AccessibleIcon<R, IV>(
    /// The accessible label for the icon. This label will be visually hidden but announced to screen reader users,
    /// similar to `alt` text for `img` tags.
    #[prop(into)]
    label: Signal<String>,
    render: R,
) -> impl IntoView
where
    R: Fn(AccessibleIconAttrs) -> IV,
    IV: IntoView,
{
    let attrs = use_accessible_icon();

    view! {
        {render(attrs)}
        <VisuallyHidden>{label}</VisuallyHidden>
    }
}
