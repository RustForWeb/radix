use leptos::{html::ElementDescriptor, *};

#[component]
pub fn Primitive<El: ElementDescriptor + 'static>(
    #[prop()] element: fn() -> HtmlElement<El>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let attrs = StoredValue::new(attrs);
    let children = StoredValue::new(children);

    view! {
        <Show
            when=move || as_child.get().unwrap_or_default()
            fallback=move || element().attrs(attrs.get_value()).child(children.get_value().into_view()).into_view()
        >
            {map_children(children.get_value()().as_children(), attrs.get_value())}
        </Show>
    }
}

fn map_children(children: &[View], attrs: Vec<(&'static str, Attribute)>) -> View {
    children
        .iter()
        .map(|child| match child {
            View::Element(element) => element
                .clone()
                .into_html_element()
                .attrs(attrs.clone())
                .into_view(),
            View::Component(component) => map_children(&component.children, attrs.clone()),
            _ => child.into_view(),
        })
        .collect_view()
}
