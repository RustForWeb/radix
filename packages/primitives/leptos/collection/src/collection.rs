// TODO: remove
#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::marker::PhantomData;

use leptos::{
    html::{AnyElement, ElementDescriptor},
    *,
};

#[derive(Clone)]
struct CollectionContextValue<ItemElement: ElementDescriptor + 'static, ItemData: Clone + 'static> {
    collection_ref: NodeRef<AnyElement>,
    item_map: RwSignal<HashMap<String, (NodeRef<ItemElement>, ItemData)>>,
}

#[component]
pub fn CollectionProvider<ItemData: Clone + 'static>(
    children: ChildrenFn,
    #[prop(into, optional)] item_data: Option<PhantomData<ItemData>>,
) -> impl IntoView {
    let context_value = CollectionContextValue::<AnyElement, ItemData> {
        collection_ref: create_node_ref(),
        item_map: create_rw_signal(HashMap::new()),
    };

    view! {
        <Provider value=context_value>
            {children()}
        </Provider>
    }
}
