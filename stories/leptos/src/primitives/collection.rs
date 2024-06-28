// TODO: remove
#![allow(dead_code, unused_variables)]

use std::marker::PhantomData;

use leptos::*;
use radix_leptos_collection::*;

#[component]
pub fn Basic() -> impl IntoView {
    view! {
        <List>
            <Item>Red</Item>
            <Item disabled=true>Green></Item>
            <Item>Blue</Item>
            <LogItems />
        </List>
    }
}

#[derive(Clone)]
struct ItemData {
    disabled: Signal<bool>,
}

const ITEM_DATA_PHANTHOM: PhantomData<ItemData> = PhantomData;

#[component]
fn List(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <CollectionProvider item_data=ITEM_DATA_PHANTHOM>
            <CollectionSlot item_data=ITEM_DATA_PHANTHOM>
                <ul style:width="200px">
                    {children.with_value(|children| children())}
                </ul>
            </CollectionSlot>
        </CollectionProvider>
    }
}

#[component]
fn Item(#[prop(into, optional)] disabled: MaybeProp<bool>, children: ChildrenFn) -> impl IntoView {
    let item_data = ItemData {
        disabled: Signal::derive(move || disabled.get().unwrap_or(false)),
    };

    view! {
        <CollectionItemSlot item_data=item_data>
            <li style:opacity=move || disabled.get().map(|_| "0.3")>
                {children()}
            </li>
        </CollectionItemSlot>
    }
}

#[component]
fn LogItems() -> impl IntoView {
    view! {}
}
