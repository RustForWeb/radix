use leptos::{
    attr::{aria_hidden, custom::custom_attribute},
    prelude::*,
};
use radix_leptos_visually_hidden::VisuallyHidden;

#[component]
pub fn AccessibleIcon(
    /// The accessible label for the icon. This label will be visually hidden but announced to screen reader users, similar to `alt` text for `img` tags.
    #[prop(into)]
    label: Signal<String>,
    children: TypedChildren<impl IntoView + 'static>,
) -> impl IntoView {
    let label = Signal::derive(move || label.get());
    view! {
        <>
            {children
                .into_inner()()
                // Accessibility
                .add_any_attr(aria_hidden("true"))
                // See: https://allyjs.io/tutorials/focusing-in-svg.html#making-svg-elements-focusable
                .add_any_attr(custom_attribute("focusable", "false"))}
            <VisuallyHidden>{label.get()}</VisuallyHidden>
        </>
    }
}
