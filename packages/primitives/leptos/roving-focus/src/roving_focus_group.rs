// TODO: remove
#![allow(dead_code, unused_variables)]

use std::marker::PhantomData;
use std::rc::Rc;

use leptos::{
    ev::{Event, FocusEvent, KeyboardEvent, MouseEvent},
    html::AnyElement,
    *,
};
use radix_leptos_collection::CollectionProvider;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_direction::{use_direction, Direction};
use radix_leptos_primitive::{compose_callbacks, Primitive};
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    CustomEvent, CustomEventInit,
};

const ENTRY_FOCUS: &str = "rovingFocusGroup.onEntryFocus";

#[derive(Clone, Debug)]
struct ItemData {
    id: String,
    focusable: Signal<bool>,
    active: Signal<bool>,
}

const ITEM_DATA_PHANTHOM: PhantomData<ItemData> = PhantomData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl From<Orientation> for String {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Horizontal => "horizontal".into(),
            Orientation::Vertical => "vertical".into(),
        }
    }
}

impl IntoAttribute for Orientation {
    fn into_attribute(self) -> Attribute {
        let s: String = self.into();
        Attribute::String(s.into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

#[derive(Clone, Debug)]
struct RovingContextValue {
    orientation: Signal<Option<Orientation>>,
    dir: Signal<Direction>,
    r#loop: Signal<bool>,
    current_tab_stop_id: Signal<Option<String>>,
    on_item_focus: Callback<String>,
    on_item_shift_tab: Callback<()>,
    on_focusable_item_add: Callback<()>,
    on_focusable_item_remove: Callback<()>,
}

#[component]
pub fn RovingFocusGroup(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] on_mouse_down: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let attrs = StoredValue::new(attrs);
    let children = StoredValue::new(children);

    // TODO: Collection.Provider, Collection.Slot

    view! {
        <CollectionProvider item_data=ITEM_DATA_PHANTHOM>
            <RovingFocusGroupImpl
                orientation=orientation
                dir=dir
                r#loop=r#loop
                on_mouse_down=on_mouse_down
                on_focus=on_focus
                as_child=as_child
                node_ref=node_ref
                attrs=attrs.get_value()
            >
                {children.with_value(|children| children())}
            </RovingFocusGroupImpl>
        </CollectionProvider>
    }
}

#[component]
fn RovingFocusGroupImpl(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] on_entry_focus: Option<Callback<Event>>,
    #[prop(into, optional)] on_mouse_down: Option<Option<Callback<MouseEvent>>>,
    #[prop(into, optional)] on_focus: Option<Option<Callback<FocusEvent>>>,
    #[prop(into, optional)] on_blur: Option<Option<Callback<FocusEvent>>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let orientation = Signal::derive(move || orientation.get());
    let r#loop = Signal::derive(move || r#loop.get().unwrap_or(false));

    let group_ref = create_node_ref::<AnyElement>();
    let composed_refs = use_composed_refs(vec![node_ref, group_ref]);
    let direction = use_direction(dir);
    // TODO: replace with use_controllable_state
    let (current_tab_stop_id, set_current_tab_stop_id) = create_signal::<Option<String>>(None);
    let (is_tabbing_back_out, set_is_tabbing_back_out) = create_signal(false);
    let is_click_focus = create_rw_signal(false);
    let (focusable_items_count, set_focusable_items_count) = create_signal(0);

    let handle_entry_focus: Rc<Closure<dyn Fn(Event)>> =
        Rc::new(Closure::new(move |event: Event| {
            if let Some(on_entry_focus) = on_entry_focus {
                on_entry_focus.call(event);
            }
        }));
    let cleanup_hanle_entry_focus = handle_entry_focus.clone();

    create_effect(move |_| {
        if let Some(node) = group_ref.get() {
            node.add_event_listener_with_callback(
                ENTRY_FOCUS,
                (*handle_entry_focus).as_ref().unchecked_ref(),
            )
            .expect("Entry focus event listener should be added.");
        }
    });

    on_cleanup(move || {
        if let Some(node) = group_ref.get() {
            node.remove_event_listener_with_callback(
                ENTRY_FOCUS,
                (*cleanup_hanle_entry_focus).as_ref().unchecked_ref(),
            )
            .expect("Entry focus event listener should be removed.");
        }
    });

    let roving_context_value = RovingContextValue {
        orientation,
        dir: direction,
        r#loop,
        current_tab_stop_id: current_tab_stop_id.into(),
        on_item_focus: Callback::new(move |tab_stop_id| {
            set_current_tab_stop_id.set(Some(tab_stop_id))
        }),
        on_item_shift_tab: Callback::new(move |_| set_is_tabbing_back_out.set(true)),
        on_focusable_item_add: Callback::new(move |_| {
            set_focusable_items_count.update(|focusable_items_count| *focusable_items_count += 1)
        }),
        on_focusable_item_remove: Callback::new(move |_| {
            set_focusable_items_count.update(|focusable_items_count| *focusable_items_count -= 1)
        }),
    };

    let mut attrs = attrs.clone();
    attrs.extend([
        (
            "tab-index",
            (move || match is_tabbing_back_out.get() || focusable_items_count.get() == 0 {
                true => "-1",
                false => "0",
            })
            .into_attribute(),
        ),
        (
            "data-orientation",
            (move || orientation.get()).into_attribute(),
        ),
        // TODO: style
    ]);

    view! {
        <Provider value={roving_context_value}>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_refs
                attrs=attrs
                on:mousedown=compose_callbacks(on_mouse_down.flatten(), Some(Callback::new(move |_: MouseEvent| {
                    is_click_focus.set(true);
                })), None)
                on:focus=compose_callbacks(on_focus.flatten(), Some(Callback::new(move |event: FocusEvent| {
                    // We normally wouldn't need this check, because we already check
                    // that the focus is on the current target and not bubbling to it.
                    // We do this because Safari doesn't focus buttons when clicked, and
                    // instead, the wrapper will get focused and not through a bubbling event.
                    let is_keyboard_focus = !is_click_focus.get();

                    if event.target() == event.current_target() && is_keyboard_focus && !is_tabbing_back_out.get() {
                        let entry_focus_event = CustomEvent::new_with_event_init_dict(ENTRY_FOCUS, CustomEventInit::new().bubbles(false).cancelable(true)).expect("Entry focus event should be instantiated.");
                        event.current_target().expect("Event should have current target.").dispatch_event(&entry_focus_event).expect("Entry focus event should be dispatched.");

                        if !entry_focus_event.default_prevented() {
                            // TODO
                        }
                    }

                    is_click_focus.set(false);
                })), None)
                on:blur=compose_callbacks(on_blur.flatten(), Some(Callback::new(move |_: FocusEvent| {
                    set_is_tabbing_back_out.set(false);
                })), None)
            >
                {children()}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn RovingFocusGroupItem(
    #[prop(into, optional)] tab_stop_id: MaybeProp<String>,
    #[prop(into, optional)] focusable: MaybeProp<bool>,
    #[prop(into, optional)] active: MaybeProp<bool>,
    #[prop(into, optional)] on_mouse_down: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let focusable = Signal::derive(move || focusable.get().unwrap_or(true));
    let active = Signal::derive(move || active.get().unwrap_or(false));

    // TODO
    // let auto_id = use_id();
    let auto_id = Signal::derive(move || "".to_string());
    let id = Signal::derive(move || tab_stop_id.get().unwrap_or(auto_id.get()));
    let context = expect_context::<RovingContextValue>();
    let is_current_tab_stop = Signal::derive(move || {
        context
            .current_tab_stop_id
            .get()
            .is_some_and(|current_tab_stop_id| current_tab_stop_id == id.get())
    });
    // TODO: let getItems =

    let added = create_rw_signal(false);
    create_effect(move |_| {
        if focusable.get() {
            context.on_focusable_item_add.call(());
            added.set(true);
        }
    });
    on_cleanup(move || {
        if added.get() {
            context.on_focusable_item_remove.call(());
        }
    });

    let mut attrs = attrs.clone();
    attrs.extend([
        (
            "tab-index",
            (move || match is_current_tab_stop.get() {
                true => "0",
                false => "-1",
            })
            .into_attribute(),
        ),
        (
            "data-orientation",
            (move || context.orientation.get()).into_attribute(),
        ),
    ]);

    // TODO: Collection.ItemSlot
    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attrs=attrs
            on:mousedown=compose_callbacks(on_mouse_down, Some(Callback::new(move |event: MouseEvent| {
                // We prevent focusing non-focusable items on `mousedown`.
                // Even though the item has `tab-index="-1"``, that only means take it out of the tab order.
                if !focusable.get() {
                    event.prevent_default();
                } else {
                    // Safari doesn't focus a button when clicked, so we run our logic on mousedown also.
                    context.on_item_focus.call(id.get());
                }
            })), None)
            on:focus=compose_callbacks(on_focus, Some(Callback::new(move |_: FocusEvent| {
                context.on_item_focus.call(id.get());
            })), None)
            on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: KeyboardEvent| {
                if event.key() == "Tab" && event.shift_key() {
                    context.on_item_shift_tab.call(());
                    return;
                }

                if event.target() != event.current_target() {
                    return;
                }

                let focus_intent = get_focus_intent(&event, context.orientation.get(), Some(context.dir.get()));
                if let Some(focus_intent) = focus_intent {
                    if event.meta_key() || event.ctrl_key() || event.alt_key() || event.shift_key() {
                        return;
                    }

                    event.prevent_default();

                    // TODO
                }
            })), None)
        >
            {children()}
        </Primitive>
    }
}

fn get_direction_aware_key(key: String, dir: Option<Direction>) -> String {
    if dir != Some(Direction::Rtl) {
        return key;
    }

    (match key.as_str() {
        "ArrowLeft" => "ArrowRight",
        "ArrowRight" => "Arrowleft",
        key => key,
    })
    .into()
}

#[derive(Clone, Debug, PartialEq)]
enum FocusIntent {
    First,
    Last,
    Prev,
    Next,
}

fn get_focus_intent(
    event: &KeyboardEvent,
    orientation: Option<Orientation>,
    dir: Option<Direction>,
) -> Option<FocusIntent> {
    let key = get_direction_aware_key(event.key(), dir);

    if orientation == Some(Orientation::Horizontal)
        && ["ArrowLeft", "ArrowRight"].contains(&key.as_str())
    {
        return None;
    }
    if orientation == Some(Orientation::Vertical)
        && ["ArrowUp", "ArrowDown"].contains(&key.as_str())
    {
        return None;
    }

    match key.as_str() {
        "ArrowLeft" => Some(FocusIntent::Prev),
        "ArrowUp" => Some(FocusIntent::Prev),
        "ArrowRight" => Some(FocusIntent::Next),
        "ArrowDown" => Some(FocusIntent::Next),
        "PageUp" => Some(FocusIntent::First),
        "Home" => Some(FocusIntent::First),
        "PageDown" => Some(FocusIntent::Last),
        "End" => Some(FocusIntent::Last),
        _ => None,
    }
}

fn focus_first(candidates: Vec<web_sys::HtmlElement>, prevent_scroll: Option<bool>) {
    let previously_focused_element = document().active_element();

    for candidate in candidates {
        // If focus is already where we want to go, we don't want to keep going through the candidates.
        if previously_focused_element.as_ref() == candidate.dyn_ref::<web_sys::Element>() {
            return;
        }

        // TODO: focus options with prevent_scroll
        candidate.focus().expect("Element should be focused.");

        if document().active_element() != previously_focused_element {
            return;
        }
    }
}

/// Wraps an array around itself at a given start index.
fn wrap_array<T: Clone>(array: &mut [T], start_index: usize) -> &[T] {
    array.rotate_right(start_index);
    array
}
