use yew::prelude::*;

#[hook]
pub fn use_composed_refs(refs: Vec<NodeRef>) -> NodeRef {
    let composed_ref = use_node_ref();

    use_effect({
        let composed_ref = composed_ref.clone();

        move || {
            log::info!("update composed ref {:?}", composed_ref);
            let _node = composed_ref.get();
            for r#_ref in &refs {
                // r#ref.set(node.clone());
            }
        }
    });

    composed_ref
}
