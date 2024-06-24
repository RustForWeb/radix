// TODO: remove
#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

use leptos::{
    html::{AnyElement, ElementDescriptor},
    *,
};

#[derive(Clone)]
struct CollectionContextValue<ItemElement: ElementDescriptor + 'static, ItemData: 'static> {
    collection_ref: NodeRef<AnyElement>,
    item_map: RwSignal<HashMap<String, (NodeRef<ItemElement>, ItemData)>>,
}

#[component]
pub fn CollectionProvider(children: ChildrenFn) -> impl IntoView {
    // TODO: generics
    let context_value = CollectionContextValue::<AnyElement, ()> {
        collection_ref: create_node_ref(),
        item_map: create_rw_signal(HashMap::new()),
    };

    view! {
        <Provider value=context_value>
            {children()}
        </Provider>
    }
}
