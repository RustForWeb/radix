use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashMap, fmt::Debug};

use nanoid::nanoid;
// use radix_yew_compose_refs::use_composed_refs;
use radix_yew_slot::Slot;
use yew::prelude::*;
use yew::virtual_dom::{AttributeOrProperty, Attributes};
use yew_attrs::Attrs;

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
    let context_value = use_memo((), {
        // TODO: remove after removing use_effect
        let collection_ref = collection_ref.clone();

        move |_| CollectionContextValue::<ItemData> {
            collection_ref,
            item_map,
        }
    });

    // use_effect(move || {
    //     log::info!("provider collection ref {:?}", collection_ref);
    // });

    html! {
        <ContextProvider<CollectionContextValue<ItemData>> context={(*context_value).clone()}>
            {props.children.clone()}
        </ContextProvider<CollectionContextValue<ItemData>>>
    }
}

#[derive(PartialEq, Properties)]
pub struct CollectionSlotProps {
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn CollectionSlot<ItemData: Clone + PartialEq + 'static>(props: &CollectionSlotProps) -> Html {
    let context =
        use_context::<CollectionContextValue<ItemData>>().expect("Collection context required.");
    // let composed_ref = use_composed_refs(vec![props.node_ref.clone(), context.collection_ref]);
    let composed_ref = use_composed_ref(&[props.node_ref.clone(), context.collection_ref]);

    html! {
        <Slot node_ref={composed_ref} attrs={props.attrs.clone()}>
            {props.children.clone()}
        </Slot>
    }
}

const ITEM_DATA_ATTR: &str = "data-radix-collection-item";

#[derive(PartialEq, Properties)]
pub struct CollectionItemSlotProps<ItemData: Clone + Debug + PartialEq + 'static> {
    #[prop_or_default]
    pub item_data: Option<ItemData>,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn CollectionItemSlot<ItemData: Clone + Debug + PartialEq + 'static>(
    props: &CollectionItemSlotProps<ItemData>,
) -> Html {
    let id = use_state_eq(CollectionItemId::new);
    let item_ref = use_node_ref();
    // let composed_ref = use_composed_refs(vec![props.node_ref.clone(), item_ref.clone()]);
    let composed_ref = use_composed_ref(&[props.node_ref.clone(), item_ref.clone()]);
    let context =
        use_context::<CollectionContextValue<ItemData>>().expect("Collection context required.");

    use_effect({
        let item_data = props.item_data.clone();
        let item_ref = item_ref.clone();

        move || {
            log::info!("item slot add {:?} {:?}", item_data, item_ref);

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
                log::info!("item slot remove {:?}", *id);
                item_map.borrow_mut().remove(&*id);
            }
        }
    });

    // use_effect_with((props.item_data.clone(), id, context.item_map), {
    //     let item_ref = item_ref.clone();

    //     move |(item_data, id, item_map)| {
    //         if let Some(item_data) = item_data {
    //             item_map.borrow_mut().insert(
    //                 (**id).clone(),
    //                 CollectionItemValue {
    //                     r#ref: item_ref,
    //                     data: item_data.clone(),
    //                 },
    //             );
    //         }

    //         let id = id.clone();
    //         let item_map = item_map.clone();
    //         move || {
    //             item_map.borrow_mut().remove(&*id);
    //         }
    //     }
    // });

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(Attrs::new(
                Attributes::IndexMap(Rc::new(
                    [(
                        AttrValue::from(ITEM_DATA_ATTR),
                        AttributeOrProperty::Attribute(AttrValue::from("")),
                    )]
                    .into(),
                )),
                Default::default(),
                Default::default(),
                Default::default(),
            ))
            .expect("Attributes should be merged.")
    });

    html! {
        <Slot node_ref={composed_ref} attrs={(*attrs).clone()}>
            {props.children.clone()}
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

#[hook]
pub fn use_collection<ItemData>() -> Callback<(), Vec<CollectionItemValue<ItemData>>>
where
    ItemData: Clone + Debug + PartialEq + 'static,
{
    let context =
        use_context::<CollectionContextValue<ItemData>>().expect("Collection context required.");

    // use_effect_with(context.collection_ref.clone(), |collection_ref| {
    //     log::info!("use effect with {:?}", collection_ref);
    // });

    // let collection_node = use_state_eq(|| None);
    // use_effect({
    //     let collection_node = collection_node.clone();

    //     move || {
    //         log::info!("use collection {:?}", context.collection_ref);
    //         collection_node.set(context.collection_ref.cast::<web_sys::Element>());
    //     }
    // });

    // let get_items = use_callback(
    //     (collection_node, context.item_map),
    //     |_, (collection_node, item_map)| {
    //         log::info!(
    //             "get items collection ref {:?} | item map {:?}",
    //             // collection_ref,
    //             **collection_node,
    //             item_map.borrow()
    //         );

    //         // if let Some(collection_node) = collection_ref.cast::<web_sys::Element>() {
    //         if let Some(collection_node) = collection_node.as_ref() {
    //             let ordered_nodes = node_list_to_vec(
    //                 collection_node
    //                     .query_selector_all(format!("[{ITEM_DATA_ATTR}]").as_str())
    //                     .expect("Node should be queried."),
    //             );

    //             let mut ordered_items = item_map.borrow().values().cloned().collect::<Vec<_>>();
    //             ordered_items.sort_by(|a, b| {
    //                 let index_a = ordered_nodes.iter().position(|node| {
    //                     let a: &web_sys::Node =
    //                         &a.r#ref.get().expect("Node ref should have element.");
    //                     node == a
    //                 });
    //                 let index_b = ordered_nodes.iter().position(|node| {
    //                     let b: &web_sys::Node =
    //                         &b.r#ref.get().expect("Node ref should have element.");
    //                     node == b
    //                 });

    //                 index_a.cmp(&index_b)
    //             });

    //             ordered_items
    //         } else {
    //             vec![]
    //         }
    //     },
    // );

    let get_items = use_callback(
        (context.collection_ref, context.item_map),
        |_, (collection_ref, item_map)| {
            log::info!(
                "get items collection ref {:?} | item map {:?}",
                collection_ref,
                item_map.borrow()
            );

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
