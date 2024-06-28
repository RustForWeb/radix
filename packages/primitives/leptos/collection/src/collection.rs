use std::collections::HashMap;
use std::marker::PhantomData;

use leptos::{html::AnyElement, *};
use nanoid::nanoid;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_slot::Slot;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct CollectionItemId(String);

impl CollectionItemId {
    fn new() -> Self {
        Self(nanoid!())
    }
}

#[derive(Clone)]
pub struct CollectionItemValue<ItemData> {
    pub r#ref: NodeRef<AnyElement>,
    pub data: ItemData,
}

#[derive(Clone)]
struct CollectionContextValue<ItemData: Clone + 'static> {
    collection_ref: NodeRef<AnyElement>,
    item_map: RwSignal<HashMap<CollectionItemId, CollectionItemValue<ItemData>>>,
}

#[component]
pub fn CollectionProvider<ItemData: Clone + 'static>(
    #[allow(unused_variables)]
    #[prop(into, optional)]
    item_data: Option<PhantomData<ItemData>>,
    children: ChildrenFn,
) -> impl IntoView {
    let context_value = CollectionContextValue::<ItemData> {
        collection_ref: create_node_ref(),
        item_map: create_rw_signal(HashMap::new()),
    };

    view! {
        <Provider value=context_value>
            {children()}
        </Provider>
    }
}

#[component]
pub fn CollectionSlot<ItemData: Clone + 'static>(
    #[allow(unused_variables)]
    #[prop(into, optional)]
    item_data: Option<PhantomData<ItemData>>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<CollectionContextValue<ItemData>>();
    let composed_ref = use_composed_refs(vec![node_ref, context.collection_ref]);

    view! {
        <Slot node_ref=composed_ref attrs=attrs>
            {children()}
        </Slot>
    }
}

const ITEM_DATA_ATTR: &str = "data-radix-collection-item";

#[component]
pub fn CollectionItemSlot<ItemData: Clone + 'static>(
    #[allow(unused_variables)]
    #[prop(into, optional)]
    item_data: MaybeProp<ItemData>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let (id, _) = create_signal(CollectionItemId::new());
    let item_ref = create_node_ref();
    let composed_ref = use_composed_refs(vec![node_ref, item_ref]);
    let context = expect_context::<CollectionContextValue<ItemData>>();

    create_effect(move |_| {
        context.item_map.update(|item_map| {
            if let Some(item_data) = item_data.get() {
                item_map.insert(
                    id.get(),
                    CollectionItemValue {
                        r#ref: item_ref,
                        data: item_data,
                    },
                );
            }
        });
    });

    on_cleanup(move || {
        context.item_map.update(|item_map| {
            item_map.remove(&id.get());
        });
    });

    let mut attrs = attrs.clone();
    attrs.extend([(ITEM_DATA_ATTR, "".into_attribute())]);

    view! {
        <Slot node_ref=composed_ref attrs=attrs>
            {children()}
        </Slot>
    }
}

fn node_list_to_vec(node_list: web_sys::NodeList) -> Vec<web_sys::Node> {
    let mut nodes = vec![];
    for n in 0..node_list.length() {
        if let Some(node) = node_list.item(n) {
            nodes.push(node);
        }
    }
    nodes
}

pub fn use_collection<ItemData: Clone + 'static>() -> impl Fn() -> Vec<CollectionItemValue<ItemData>>
{
    let context = expect_context::<CollectionContextValue<ItemData>>();

    let get_items = move || {
        let collection_node = context.collection_ref.get();
        if let Some(collection_node) = collection_node {
            let ordered_nodes = node_list_to_vec(
                collection_node
                    .query_selector_all(format!("[{ITEM_DATA_ATTR}]").as_str())
                    .expect("Node should be queried."),
            );

            let mut ordered_items = context.item_map.get().into_values().collect::<Vec<_>>();
            ordered_items.sort_by(|a, b| {
                let index_a = ordered_nodes.iter().position(|node| {
                    let a: &web_sys::Node = &a.r#ref.get().expect("Node ref should have element.");
                    node == a
                });
                let index_b = ordered_nodes.iter().position(|node| {
                    let b: &web_sys::Node = &b.r#ref.get().expect("Node ref should have element.");
                    node == b
                });

                index_a.cmp(&index_b)
            });

            ordered_items
        } else {
            vec![]
        }
    };

    get_items
}
