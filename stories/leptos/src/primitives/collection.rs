use std::marker::PhantomData;

use leptos::*;
use radix_leptos_collection::*;

#[component]
pub fn Basic() -> impl IntoView {
    view! {
        <List>
            <Item>Red</Item>
            <Item disabled=true>Green</Item>
            <Item>Blue</Item>
            <LogItems />
        </List>
    }
}

#[component]
pub fn WithElementsInBetween() -> impl IntoView {
    view! {
        <List>
            <div style:font-variant="small-caps">Colors</div>
            <Item>Red</Item>
            <Item disabled=true>Green</Item>
            <Item>Blue</Item>
            <div style:font-variant="small-caps">Words</div>
            <Item>Hello</Item>
            <Item>World</Item>
            <LogItems />
        </List>
    }
}
#[component]
fn Tomato() -> impl IntoView {
    view! {
        <Item attr:style="color: tomato;">Tomato</Item>
    }
}

#[component]
pub fn WithWrappedItem() -> impl IntoView {
    view! {
        <List>
            <Item>Red</Item>
            <Item disabled=true>Green</Item>
            <Item>Blue</Item>
            <Tomato />
            <LogItems />
        </List>
    }
}

#[component]
pub fn WithFragment() -> impl IntoView {
    let countries = move || {
        view! {
            <Item>France</Item>
            <Item disabled=true>UK</Item>
            <Item>Spain</Item>
        }
    };

    view! {
        <List>
            {countries}
            <LogItems />
        </List>
    }
}

#[component]
pub fn DynamicInsertion() -> impl IntoView {
    let (has_tomato, set_has_tomato) = create_signal(false);

    view! {
        <button on:click=move |_| set_has_tomato.set(!has_tomato.get())>
            {move || match has_tomato.get() {
                true => "Remove",
                false => "Add"
            }}
        </button>

        <List>
            <WrappedItems has_tomato=has_tomato />
            <LogItems />
        </List>
    }
}

#[component]
fn WrappedItems(#[prop(into)] has_tomato: Signal<bool>) -> impl IntoView {
    view! {
        <Item>Red</Item>
        <Show when=move || has_tomato.get()>
            <Tomato />
        </Show>
        <Item disabled=true>Green</Item>
        <Item>Blue</Item>
    }
}

#[component]
pub fn WithChangingItem() -> impl IntoView {
    let (is_disabled, set_is_disabled) = create_signal(false);

    view! {
        <button on:click=move |_| set_is_disabled.set(!is_disabled.get())>
            {move || match is_disabled.get() {
                true => "Enable",
                false => "Disable"
            }}
        </button>

        <List>
            <Item>Red</Item>
            <Item disabled=is_disabled>Green</Item>
            <Item>Blue</Item>
            <LogItems />
        </List>
    }
}

#[component]
pub fn Nested() -> impl IntoView {
    view! {
        <List>
            <Item>1</Item>
            <Item>
                2
                <List>
                    <Item>2.1</Item>
                    <Item>2.2</Item>
                    <Item>2.3</Item>
                    <LogItems name="items inside 2".into() />
                </List>
            </Item>
            <Item>3</Item>
            <LogItems name="top-level items".into() />
        </List>
    }
}

#[derive(Clone, Debug)]
struct ItemData {
    #[allow(dead_code)]
    disabled: bool,
}

const ITEM_DATA_PHANTHOM: PhantomData<ItemData> = PhantomData;

#[component]
fn List(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <CollectionProvider item_data_type=ITEM_DATA_PHANTHOM>
            <CollectionSlot item_data_type=ITEM_DATA_PHANTHOM>
                <ul style:width="200px">
                    {children.with_value(|children| children())}
                </ul>
            </CollectionSlot>
        </CollectionProvider>
    }
}

#[component]
fn Item(#[prop(into, optional)] disabled: MaybeProp<bool>, children: ChildrenFn) -> impl IntoView {
    let item_data = Signal::derive(move || ItemData {
        disabled: disabled.get().unwrap_or(false),
    });

    view! {
        <CollectionItemSlot item_data_type=ITEM_DATA_PHANTHOM item_data=item_data>
            <li style:opacity=move || disabled.get().unwrap_or(false).then_some("0.3")>
                {children()}
            </li>
        </CollectionItemSlot>
    }
}

#[component]
fn LogItems(#[prop(default = "items".to_string())] name: String) -> impl IntoView {
    let get_items = use_collection::<ItemData>();

    Effect::new(move |_| {
        log::info!("{} {:?}", name, get_items());
    });

    view! {}
}
