use leptos::{html::AnyElement, *};
use radix_leptos_primitive::Primitive;

#[component]
pub fn VisuallyHidden(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    // TODO: replace with style:<name> attributes once they work properly in Leptos (probably in 0.7?)
    let mut attrs = attrs.clone();
    attrs.extend([
        ("style", "position: absolute; border: 0px; width: 1px; height: 1px; padding: 0px; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; word-wrap: normal;".into_attribute()
    )]);

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attrs=attrs
        >
            {children()}
        </Primitive>
    }
}
