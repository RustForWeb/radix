use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashMap, fmt::Debug};

use nanoid::nanoid;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct CollectionItemId(String);

impl CollectionItemId {
    fn new() -> Self {
        Self(nanoid!())
    }
}

#[derive(Clone, PartialEq)]
pub struct CollectionItemValue<ItemData> {
    pub r#ref: NodeRef,
    pub data: ItemData,
}

impl<ItemData: Debug> Debug for CollectionItemValue<ItemData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CollectionItemValue")
            .field("data", &self.data)
            .finish()
    }
}

#[derive(Clone, PartialEq)]
struct CollectionContextValue<ItemData: Clone + 'static> {
    collection_ref: NodeRef,
    item_map: Rc<RefCell<HashMap<CollectionItemId, CollectionItemValue<ItemData>>>>,
}

#[derive(PartialEq, Properties)]
pub struct CollectionProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn CollectionProvider<ItemData: Clone + PartialEq + 'static>(
    props: &CollectionProviderProps,
) -> Html {
    let collection_ref = use_node_ref();
    let item_map = use_mut_ref(HashMap::new);
    let context_value = use_memo((), move |_| CollectionContextValue::<ItemData> {
        collection_ref,
        item_map,
    });

    html! {
        <ContextProvider<CollectionContextValue<ItemData>> context={(*context_value).clone()}>
            {props.children.clone()}
        </ContextProvider<CollectionContextValue<ItemData>>>
    }
}

#[derive(PartialEq, Properties)]
pub struct CollectionSlotProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    pub as_child: Callback<CollectionSlotChildProps, Html>,
}

#[derive(Clone, Default, PartialEq)]
pub struct CollectionSlotChildProps {
    pub node_ref: NodeRef,
}

#[function_component]
pub fn CollectionSlot<ItemData: Clone + PartialEq + 'static>(props: &CollectionSlotProps) -> Html {
    let context =
        use_context::<CollectionContextValue<ItemData>>().expect("Collection context required.");
    let composed_ref = use_composed_ref(&[props.node_ref.clone(), context.collection_ref]);

    props.as_child.emit(CollectionSlotChildProps {
        node_ref: composed_ref,
    })
}

const ITEM_DATA_ATTR: &str = "data-radix-collection-item";

#[derive(PartialEq, Properties)]
pub struct CollectionItemSlotProps<ItemData: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub item_data: Option<ItemData>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    pub as_child: Callback<CollectionItemSlotChildProps, Html>,
}

#[derive(Clone, Default, PartialEq)]
pub struct CollectionItemSlotChildProps {
    pub node_ref: NodeRef,
    pub data_radix_collection_item: String,
}

#[function_component]
pub fn CollectionItemSlot<ItemData: Clone + PartialEq + 'static>(
    props: &CollectionItemSlotProps<ItemData>,
) -> Html {
    let id = use_state_eq(CollectionItemId::new);
    let item_ref = use_node_ref();
    let composed_ref = use_composed_ref(&[props.node_ref.clone(), item_ref.clone()]);
    let context =
        use_context::<CollectionContextValue<ItemData>>().expect("Collection context required.");

    use_effect({
        let item_data = props.item_data.clone();
        let item_ref = item_ref.clone();

        move || {
            if let Some(item_data) = item_data {
                context.item_map.borrow_mut().insert(
                    (*id).clone(),
                    CollectionItemValue {
                        r#ref: item_ref,
                        data: item_data,
                    },
                );
            }

            let id = id.clone();
            let item_map = context.item_map.clone();
            move || {
                item_map.borrow_mut().remove(&*id);
            }
        }
    });

    props.as_child.emit(CollectionItemSlotChildProps {
        node_ref: composed_ref,
        data_radix_collection_item: "".into(),
    })
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

#[hook]
pub fn use_collection<ItemData>() -> Callback<(), Vec<CollectionItemValue<ItemData>>>
where
    ItemData: Clone + PartialEq + 'static,
{
    let context =
        use_context::<CollectionContextValue<ItemData>>().expect("Collection context required.");

    let get_items = use_callback(
        (context.collection_ref, context.item_map),
        |_, (collection_ref, item_map)| {
            if let Some(collection_node) = collection_ref.cast::<web_sys::Element>() {
                let ordered_nodes = node_list_to_vec(
                    collection_node
                        .query_selector_all(format!("[{ITEM_DATA_ATTR}]").as_str())
                        .expect("Node should be queried."),
                );

                let mut ordered_items = item_map.borrow().values().cloned().collect::<Vec<_>>();
                ordered_items.sort_by(|a, b| {
                    let index_a = ordered_nodes.iter().position(|node| {
                        let a: &web_sys::Node =
                            &a.r#ref.get().expect("Node ref should have element.");
                        node == a
                    });
                    let index_b = ordered_nodes.iter().position(|node| {
                        let b: &web_sys::Node =
                            &b.r#ref.get().expect("Node ref should have element.");
                        node == b
                    });

                    index_a.cmp(&index_b)
                });

                ordered_items
            } else {
                vec![]
            }
        },
    );

    get_items
}
