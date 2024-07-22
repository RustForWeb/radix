use leptos::{html::AnyElement, *};
use radix_leptos_primitive::Primitive;

#[component]
pub fn Arrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let width = move || width.get().unwrap_or(10.0);
    let height = move || height.get().unwrap_or(5.0);
    let children = StoredValue::new(children);

    let mut attrs = attrs.clone();
    attrs.extend([
        ("width", width.into_attribute()),
        ("height", height.into_attribute()),
        ("viewBox", "0 0 30 10".into_attribute()),
        ("preserveAspectRatio", "none".into_attribute()),
    ]);

    view! {
        <Primitive
            element=svg::svg
            as_child=as_child
            node_ref=node_ref
            attrs=attrs
        >
            <Show
                when=move || as_child.get().unwrap_or_default()
                fallback=move || view!{
                    <polygon points="0,0 30,0 15,10" />
                }
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Show>
        </Primitive>
    }
}
