use leptos::{
    html::{AnyElement, ElementDescriptor},
    *,
};

#[component]
pub fn Primitive<El: ElementDescriptor + 'static>(
    #[prop()] element: fn() -> HtmlElement<El>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let attrs = StoredValue::new(attrs);
    let children = StoredValue::new(children);

    view! {
        <Show
            when=move || as_child.get().unwrap_or_default()
            fallback=move || element()
                .attrs(attrs.get_value())
                .child(children.with_value(|children| children()).into_view())
                .into_any()
                .node_ref(node_ref)
                .into_view()
        >
            {map_children(children.with_value(|children| children()).as_children(), node_ref, attrs.get_value())}
        </Show>
    }
}

fn map_children(
    children: &[View],
    node_ref: NodeRef<AnyElement>,
    attrs: Vec<(&'static str, Attribute)>,
) -> View {
    children
        .iter()
        .map(|child| match child {
            View::Element(element) => element
                .clone()
                .into_html_element()
                .node_ref(node_ref)
                .attrs(attrs.clone())
                .into_view(),
            View::Component(component) => {
                map_children(&component.children, node_ref, attrs.clone())
            }
            _ => child.into_view(),
        })
        .collect_view()
}
