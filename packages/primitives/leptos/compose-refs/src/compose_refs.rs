use leptos::{html::ElementDescriptor, Effect, NodeRef};

// TODO: Support ref functions? These take the node as argument.

fn compose_refs<T: ElementDescriptor + Clone + 'static>(refs: Vec<NodeRef<T>>) -> NodeRef<T> {
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

pub fn use_composed_refs<T: ElementDescriptor + Clone + 'static>(
    refs: Vec<NodeRef<T>>,
) -> NodeRef<T> {
    compose_refs(refs)
}
