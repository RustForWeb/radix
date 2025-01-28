use leptos::web_sys::Element;
use leptos::{html::ElementType, prelude::*, tachys::html::node_ref::NodeRefContainer};

fn compose_refs<T: ElementType<Output = Element> + Clone + 'static>(
    refs: Vec<NodeRef<T>>,
) -> NodeRef<T> {
    let composed_ref = NodeRef::new();

    Effect::new(move |_| {
        if let Some(node) = composed_ref.get() {
            for r#ref in &refs {
                r#ref.load(&node);
            }
        }
    });

    composed_ref
}

pub fn use_composed_refs<T: ElementType<Output = Element> + Clone + 'static>(
    refs: Vec<NodeRef<T>>,
) -> NodeRef<T> {
    compose_refs(refs)
}
