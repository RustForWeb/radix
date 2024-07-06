use leptos::*;
use radix_leptos_visually_hidden::VisuallyHidden;

#[component]
pub fn AccessibleIcon(
    /// The accessible label for the icon. This label will be visually hidden but announced to screen reader users,
    /// similar to `alt` text for `img` tags.
    #[prop(into)]
    label: MaybeSignal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let label = Signal::derive(move || label.get());

    view! {
        {children.map(|children| map_children(children().as_children()))}
        <VisuallyHidden>{label}</VisuallyHidden>
    }
}

fn map_children(children: &[View]) -> View {
    children
        .iter()
        .map(|child| match child {
            View::Element(element) => element
                .clone()
                .into_html_element()
                // Accessibility
                .attr("aria-hidden", "true")
                // See: https://allyjs.io/tutorials/focusing-in-svg.html#making-svg-elements-focusable
                .attr("focusable", "false")
                .into_view(),
            View::Component(component) => map_children(&component.children),
            _ => child.into_view(),
        })
        .collect_view()
}
