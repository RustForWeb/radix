use yew::prelude::*;

#[hook]
pub fn use_composed_refs(refs: Vec<NodeRef>) -> NodeRef {
    let composed_ref = use_node_ref();

    use_effect_with(composed_ref.clone(), move |composed_ref| {
        let node = composed_ref.get();
        for r#ref in &refs {
            r#ref.set(node.clone());
        }
    });

    composed_ref
}
