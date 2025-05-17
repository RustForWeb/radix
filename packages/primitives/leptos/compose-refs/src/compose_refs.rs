use leptos::{html::Div, prelude::*, tachys::html::node_ref::NodeRefContainer};
use leptos_node_ref::AnyNodeRef;

fn compose_refs(refs: Vec<AnyNodeRef>) -> AnyNodeRef {
    let composed_ref = AnyNodeRef::new();

    Effect::new(move |_| {
        if let Some(node) = composed_ref.get() {
            for r#ref in &refs {
                NodeRefContainer::<Div>::load(*r#ref, &node);
                // r#ref.load_any(&node);
            }
        }
    });

    composed_ref
}

pub fn use_composed_refs(refs: Vec<AnyNodeRef>) -> AnyNodeRef {
    compose_refs(refs)
}
