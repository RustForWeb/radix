// TODO: remove
#![allow(dead_code, unused_variables)]

use std::rc::Rc;

use ev::CustomEvent;
use leptos::{
    ev::{Event, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent},
    html::AnyElement,
    wasm_bindgen::{closure::Closure, JsCast},
    web_sys::AddEventListenerOptions,
    *,
};
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_direction::{use_direction, Direction};
use radix_leptos_dismissable_layer::{
    FocusOutsideEvent, InteractOutsideEvent, PointerDownOutsideEvent,
};
use radix_leptos_focus_guards::use_focus_guards;
use radix_leptos_focus_scope::FocusScope;
use radix_leptos_popper::{Popper, PopperAnchor, PopperArrow, PopperContent};
use radix_leptos_primitive::{compose_callbacks, Primitive};
use web_sys::CustomEventInit;

const SELECTION_KEYS: [&str; 2] = ["Enter", " "];
const FIRST_KEYS: [&str; 3] = ["ArrowDown", "PageUp", "Home"];
const LAST_KEYS: [&str; 3] = ["ArrowUp", "PageDown", "End"];
const FIRST_LAST_KEYS: [&str; 6] = ["ArrowDown", "PageUp", "Home", "ArrowUp", "PageDown", "End"];

#[derive(Clone)]
struct MenuContextValue {
    open: Signal<bool>,
    content_ref: NodeRef<AnyElement>,
    on_open_change: Callback<bool>,
}

#[derive(Clone)]
struct MenuRootContextValue {
    is_using_keyboard: Signal<bool>,
    dir: Signal<Direction>,
    modal: Signal<bool>,
    on_close: Callback<()>,
}

#[component]
pub fn Menu(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let open = Signal::derive(move || open.get().unwrap_or(false));
    let modal = Signal::derive(move || modal.get().unwrap_or(true));
    let on_open_change = on_open_change.unwrap_or(Callback::new(|_| {}));

    let content_ref = create_node_ref::<AnyElement>();
    let is_using_keyboard = create_rw_signal(false);
    let direction = use_direction(dir);

    let context_value = StoredValue::new(MenuContextValue {
        open,
        content_ref,
        on_open_change,
    });
    let root_context_value = StoredValue::new(MenuRootContextValue {
        is_using_keyboard: is_using_keyboard.into(),
        dir: direction,
        modal,
        on_close: Callback::new(move |_| on_open_change.call(false)),
    });

    let handle_pointer: Rc<Closure<dyn Fn(PointerEvent)>> = Rc::new(Closure::new(move |_| {
        is_using_keyboard.set(false);
    }));
    let cleanup_handle_pointer = handle_pointer.clone();

    let handle_key_down: Rc<Closure<dyn Fn(KeyboardEvent)>> = Rc::new(Closure::new(move |_| {
        is_using_keyboard.set(true);

        document()
            .add_event_listener_with_callback_and_add_event_listener_options(
                "pointerdown",
                (*handle_pointer).as_ref().unchecked_ref(),
                AddEventListenerOptions::new().capture(true).once(true),
            )
            .expect("Pointer down event listener should be added.");
        document()
            .add_event_listener_with_callback_and_add_event_listener_options(
                "pointermove",
                (*handle_pointer).as_ref().unchecked_ref(),
                AddEventListenerOptions::new().capture(true).once(true),
            )
            .expect("Pointer move event listener should be added.");
    }));
    let cleanup_handle_key_down = handle_key_down.clone();

    create_effect(move |_| {
        // Capture phase ensures we set the boolean before any side effects execute
        // in response to the key or pointer event as they might depend on this value.
        document()
            .add_event_listener_with_callback_and_add_event_listener_options(
                "keydown",
                (*handle_key_down).as_ref().unchecked_ref(),
                AddEventListenerOptions::new().capture(true),
            )
            .expect("Key down event listener should be added.");
    });

    on_cleanup(move || {
        document()
            .remove_event_listener_with_callback_and_bool(
                "keydown",
                (*cleanup_handle_key_down).as_ref().unchecked_ref(),
                true,
            )
            .expect("Key down event listener should be removed.");

        document()
            .remove_event_listener_with_callback_and_bool(
                "pointerdown",
                (*cleanup_handle_pointer).as_ref().unchecked_ref(),
                true,
            )
            .expect("Pointer down event listener should be removed.");

        document()
            .remove_event_listener_with_callback_and_bool(
                "pointermove",
                (*cleanup_handle_pointer).as_ref().unchecked_ref(),
                true,
            )
            .expect("Pointer move event listener should be removed.");
    });

    view! {
        <Popper>
            <Provider value=context_value>
                <Provider value=root_context_value>
                    {children.with_value(|children| children())}
                </Provider>
            </Provider>
        </Popper>
    }
}

#[component]
pub fn MenuAnchor(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <PopperAnchor as_child=as_child node_ref=node_ref attrs=attrs>
            {children()}
        </PopperAnchor>
    }
}

#[component]
pub fn MenuPortal(
    /// Specify a container element to portal the content into.
    #[prop(into, optional)]
    container: MaybeProp<web_sys::Element>,
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    // TODO: portal
    // view! {}
    children()
}

#[derive(Clone)]
struct MenuContentContextValue {
    on_item_enter: Callback<PointerEvent>,
    on_item_leave: Callback<PointerEvent>,
    on_trigger_leave: Callback<PointerEvent>,
    search: RwSignal<String>,
    pointer_grace_timer: RwSignal<u64>,
    on_pointer_grace_intent_change: Callback<Option<GraceIntent>>,
}

#[component]
pub fn MenuContent(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let attrs: StoredValue<Vec<(&str, Attribute)>> = StoredValue::new(attrs);
    let children = StoredValue::new(children);

    let root_context = expect_context::<MenuRootContextValue>();

    view! {
        // TODO: wrapper components
        <Show
            when=move || root_context.modal.get()
            fallback=move || view!{
                <MenuRootContentNonModal attrs=attrs.get_value()>
                    {children.with_value(|children| children())}
                </MenuRootContentNonModal>
            }
        >
            <MenuRootContentModal as_child=as_child node_ref=node_ref attrs=attrs.get_value()>
                {children.with_value(|children| children())}
            </MenuRootContentModal>
        </Show>
    }
}

#[component]
fn MenuRootContentModal(
    #[prop(into, optional)] on_focus_outside: Option<Callback<FocusOutsideEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<MenuContextValue>();
    let content_ref = create_node_ref::<AnyElement>();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);

    // Hide everything from ARIA except the `MenuContent`.
    create_effect(move |_| {
        if let Some(content) = content_ref.get() {
            // TODO: imported from `aria-hidden` in JS.
            // hide_others(content);
        }
    });

    view! {
        <MenuContentImpl
            // We make sure we're not trapping once it's been closed (closed != unmounted when animating out).
            trap_focus=context.open
            // Make sure to only disable pointer events when open. This avoids blocking interactions while animating out.
            disable_outside_pointer_events=context.open
            disable_outside_scroll=true
            // When focus is trapped, a `focusout` event may still happen. We make sure we don't trigger our `on_dismiss` in such case.
            on_focus_outside=compose_callbacks(on_focus_outside, Some(Callback::new(move |event: FocusOutsideEvent| {
                event.prevent_default();
            })), Some(false))
            on_dismiss=move |_| context.on_open_change.call(false)
            as_child=as_child
            node_ref=composed_refs
            attrs=attrs
        >
            {children()}
        </MenuContentImpl>
    }
}

#[component]
fn MenuRootContentNonModal(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<MenuContextValue>();

    view! {
        <MenuContentImpl
            trap_focus=false
            disable_outside_pointer_events=false
            disable_outside_scroll=false
            on_dismiss=move |_| context.on_open_change.call(false)
            as_child=as_child
            node_ref=node_ref
            attrs=attrs
        >
            {children()}
        </MenuContentImpl>
    }
}

#[component]
fn MenuContentImpl(
    /// Event handler called when auto-focusing on open. Can be prevented.
    #[prop(into, optional)]
    on_open_auto_focus: Option<Callback<Event>>,
    /// Event handler called when auto-focusing on close. Can be prevented.
    #[prop(into, optional)]
    on_close_auto_focus: Option<Callback<Event>>,
    #[prop(into, optional)] disable_outside_pointer_events: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<PointerDownOutsideEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<FocusOutsideEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<InteractOutsideEvent>>,
    #[prop(into, optional)] on_dismiss: Option<Callback<()>>,
    #[prop(into, optional)] on_key_down: Option<Callback<KeyboardEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<PointerEvent>>,
    /// Whether scrolling outside the `MenuContent` should be prevented. Defaults to `false`.
    #[prop(into, optional)]
    disable_outside_scroll: MaybeProp<bool>,
    /// Whether focus should be trapped within the `MenuContent`. Defaults to `false`.
    #[prop(into, optional)]
    trap_focus: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = expect_context::<MenuContextValue>();
    let root_context = expect_context::<MenuRootContextValue>();
    let (current_item_id, set_current_item_id) = create_signal::<Option<String>>(None);
    let content_ref = create_node_ref::<AnyElement>();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);
    let timer = create_rw_signal(0);
    let search = create_rw_signal("".to_string());
    let pointer_grace_timer = create_rw_signal(0);
    let pointer_grace_intent = create_rw_signal::<Option<GraceIntent>>(None);
    let pointer_dir = create_rw_signal(Side::Right);
    let last_pointer_x = create_rw_signal(0);

    let handle_typeahead_search = move |key: String| {
        let search = search.get() + &key;
        // TODO
        // let items = get_items().filter();
        let current_item = document().active_element();
        // let current_match =

        // TODO
    };

    on_cleanup(move || {
        window().clear_timeout_with_handle(timer.get());
    });

    // Make sure the whole tree has focus guards as our `MenuContent` may be the last element in the DOM (beacuse of the `Portal`).
    use_focus_guards();

    let is_pointer_moving_to_submenu = move |event: &PointerEvent| -> bool {
        let is_moving_towards = Some(pointer_dir.get())
            == pointer_grace_intent
                .get()
                .map(|pointer_grace_intent| pointer_grace_intent.side);
        is_moving_towards
            && is_pointer_in_grace_area(
                event,
                pointer_grace_intent
                    .get()
                    .map(|pointer_grace_intent| pointer_grace_intent.area),
            )
    };

    let content_context_value = StoredValue::new(MenuContentContextValue {
        search,
        on_item_enter: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                event.prevent_default();
            }
        }),
        on_item_leave: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                return;
            }
            if let Some(content) = content_ref.get() {
                content.focus().expect("Element should be focused.");
            }
            set_current_item_id.set(None);
        }),
        on_trigger_leave: Callback::new(move |event| {
            if is_pointer_moving_to_submenu(&event) {
                event.prevent_default();
            }
        }),
        pointer_grace_timer,
        on_pointer_grace_intent_change: Callback::new(move |intent| {
            pointer_grace_intent.set(intent);
        }),
    });

    let mut attrs = attrs.clone();
    attrs.extend([
        ("role", "menu".into_attribute()),
        ("aria-orientation", "vertical".into_attribute()),
        (
            "data-state",
            (move || get_open_state(context.open.get())).into_attribute(),
        ),
        ("data-radix-menu-content", "".into_attribute()),
        ("dir", (move || root_context.dir.get()).into_attribute()),
        // TODO: style
    ]);

    let attrs = StoredValue::new(attrs);
    let children = StoredValue::new(children);

    // TODO: ScrollLockWrapper, DismissableLayer, RovingFocusGroup.Root
    view! {
        <Provider value=content_context_value>
            <FocusScope
                as_child=true
                trapped=trap_focus
                on_mount_auto_focus=compose_callbacks(
                    on_open_auto_focus,
                    Some(Callback::new(move |event: Event| {
                        // When opening, explicitly focus the content area only and leave `onEntryFocus` in  control of focusing first item.
                        event.prevent_default();

                        if let Some(content) = content_ref.get_untracked() {
                            // TODO: focus with options doesn't exist in web-sys
                            content.focus().expect("Element should be focused");
                        }
                    })),
                    None,
                )
                on_unmount_auto_focus=on_close_auto_focus
            >
                <PopperContent
                    as_child=as_child
                    node_ref=composed_refs
                    attrs=attrs.get_value()
                    on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: KeyboardEvent| {
                        // Submenu key events bubble through portals. We only care about keys in this menu.
                        let target = event.target().map(|target| target.unchecked_into::<web_sys::HtmlElement>()).expect("Event should have target.");
                        let is_key_down_inside = target.closest("[data-radix-menu-content]").expect("Element should be able to query closest.") ==
                            event.current_target().and_then(|current_target| current_target.dyn_into::<web_sys::Element>().ok());
                        let is_modifier_key = event.ctrl_key() || event.alt_key() || event.meta_key();
                        let is_character_key = event.key().len() == 1;

                        if is_key_down_inside {
                            // Menus should not be navigated using tab key so we prevent it.
                            if event.key() == "Tab" {
                                event.prevent_default();
                            }
                            if !is_modifier_key && is_character_key {
                                handle_typeahead_search(event.key());
                            }
                        }

                        // Focus first/last item based on key pressed.
                        if content_ref.get().is_some_and(|content| *content == target) {
                            if !FIRST_LAST_KEYS.contains(&event.key().as_str()) {
                                return;
                            }

                            event.prevent_default();

                            // TODO: get_items().filter()
                            // let items = vec![];
                            let candidate_nodes: Vec<web_sys::HtmlElement> = vec![];
                            if LAST_KEYS.contains(&event.key().as_str()) {
                                // TODO
                            }
                            focus_first(candidate_nodes);
                        }

                    })), None)
                    on:blur=compose_callbacks(on_blur, Some(Callback::new(move |event: FocusEvent| {
                        // Clear search buffer when leaving the menu.
                        let target = event.target().map(|target| target.unchecked_into::<web_sys::Node>()).expect("Event should have target.");
                        let current_target = event.current_target().map(|current_target| current_target.unchecked_into::<web_sys::Node>()).expect("Event should have current target.");
                        if !current_target.contains(Some(&target)) {
                            window().clear_timeout_with_handle(timer.get());
                            search.set("".into());
                        }
                    })), None)
                    on:pointermove=compose_callbacks(on_pointer_move, Some(when_mouse(move |event: PointerEvent| {
                        let target = event.target().map(|target| target.unchecked_into::<web_sys::HtmlElement>()).expect("Event should have target.");
                        let current_target = event.current_target().map(|current_target| current_target.unchecked_into::<web_sys::Node>()).expect("Event should have current target.");
                        let pointer_x_has_changed = last_pointer_x.get() != event.client_x();

                        // We don't use `event.movementX` for this check because Safari will always return `0` on a pointer event.
                        if current_target.contains(Some(&target)) && pointer_x_has_changed {
                            let new_dir = match event.client_x() > last_pointer_x.get() {
                                true => Side::Right,
                                false => Side::Left
                            };
                            pointer_dir.set(new_dir);
                            last_pointer_x.set(event.client_x());
                        }
                    })), None)
                >
                    {children.with_value(|children| children())}
                </PopperContent>
            </FocusScope>
        </Provider>
    }
}

#[component]
pub fn MenuGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let mut attrs = attrs.clone();
    attrs.extend([("role", "group".into_attribute())]);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attrs=attrs
        >
            {children()}
        </Primitive>
    }
}

#[component]
pub fn MenuLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attrs=attrs
        >
            {children()}
        </Primitive>
    }
}

const ITEM_SELECT: &str = "menu.itemSelect";

#[component]
pub fn MenuItem(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<Event>>,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<PointerEvent>>,
    #[prop(into, optional)] on_pointer_up: Option<Callback<PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let item_ref = create_node_ref::<AnyElement>();
    let composed_refs = use_composed_refs(vec![node_ref, item_ref]);
    let root_context = expect_context::<MenuRootContextValue>();
    let content_context = expect_context::<MenuContentContextValue>();
    let is_pointer_down = create_rw_signal(false);

    let handle_select = Callback::new(move |_: MouseEvent| {
        if disabled.get() {
            return;
        }

        if let Some(item) = item_ref.get() {
            let closure: Closure<dyn Fn(Event)> = Closure::new(move |event: Event| {
                if let Some(on_select) = on_select {
                    on_select.call(event);
                }
            });

            let item_select_event = CustomEvent::new_with_event_init_dict(
                ITEM_SELECT,
                CustomEventInit::new().bubbles(true).cancelable(true),
            )
            .expect("Item select event should be instantiated.");

            item.add_event_listener_with_callback_and_add_event_listener_options(
                ITEM_SELECT,
                closure.as_ref().unchecked_ref(),
                AddEventListenerOptions::new().once(true),
            )
            .expect("Item select event listener should be added.");
            item.dispatch_event(&item_select_event)
                .expect("Item select event should be dispatched.");

            if item_select_event.default_prevented() {
                is_pointer_down.set(false);
            } else {
                root_context.on_close.call(());
            }
        }
    });

    view! {
        <MenuItemImpl
            disabled={disabled}
            as_child=as_child
            node_ref=composed_refs
            attrs=attrs
            on:click=compose_callbacks(on_click, Some(handle_select), None)
            on:pointerdown=move |event| {
                if let Some(on_pointer_down) = on_pointer_down {
                    on_pointer_down.call(event);
                }
                is_pointer_down.set(true);
            }
            on:pointerup=compose_callbacks(on_pointer_up, Some(Callback::new(move |event: PointerEvent| {
                // Pointer down can move to a different menu item which should activate it on pointer up.
                // We dispatch a click for selection to allow composition with click based triggers and to
                // prevent Firefox from getting stuck in text selection mode when the menu closes.
                if is_pointer_down.get() {
                    if let Some(current_target) = event.current_target().map(|current_target| current_target.unchecked_into::<web_sys::HtmlElement>()) {
                        current_target.click();
                    }
                }
            })), None)
            on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: KeyboardEvent| {
                let is_typing_ahead = !content_context.search.get().is_empty();
                if disabled.get() || (is_typing_ahead && event.key() == " ") {
                    return;
                }
                if SELECTION_KEYS.contains(&event.key().as_str()) {
                    let current_target = event.current_target().map(|current_target| current_target.unchecked_into::<web_sys::HtmlElement>()).expect("Event should have current target.");
                    current_target.click();

                    // We prevent default browser behaviour for selection keys as they should trigger a selection only:
                    // - prevents space from scrolling the page.
                    // - if keydown causes focus to move, prevents keydown from firing on the new target.
                    event.prevent_default();
                }
            })), None)
        >
            {children()}
        </MenuItemImpl>
    }
}

#[component]
fn MenuItemImpl(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<PointerEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let content_context = expect_context::<MenuContentContextValue>();
    let item_ref = create_node_ref::<AnyElement>();
    let composed_ref = use_composed_refs(vec![node_ref, item_ref]);
    let (is_focused, set_is_focused) = create_signal(false);

    // Get the item's `.textContent` as default strategy for typeahead `textValue`.
    let (text_content, set_text_content) = create_signal("".to_string());
    create_effect(move |_| {
        if let Some(item) = item_ref.get() {
            set_text_content.set(item.text_content().unwrap_or("".into()).trim().into());
        }
    });

    let mut attrs = attrs.clone();
    attrs.extend([
        ("role", "menuitem".into_attribute()),
        (
            "data-highlighted",
            (move || is_focused.get().then_some("")).into_attribute(),
        ),
        (
            "aria-disabled",
            (move || disabled.get().then_some("true")).into_attribute(),
        ),
        (
            "data-disabled",
            (move || disabled.get().then_some("")).into_attribute(),
        ),
    ]);

    // TODO: Collection.ItemSlot and RovingFocusGroup.Item

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=composed_ref
            attrs=attrs
            /*
             * We focus items on `pointermove` to achieve the following:
             *
             * - Mouse over an item (it focuses)
             * - Leave mouse where it is and use keyboard to focus a different item
             * - Wiggle mouse without it leaving previously focused item
             * - Previously focused item should re-focus
             *
             * If we used `mouseover`/`mouseenter` it would not re-focus when the mouse
             * wiggles. This is to match native menu implementation.
             */
            on:pointermove=compose_callbacks(on_pointer_move, Some(when_mouse(move |event| {
                if disabled.get() {
                    content_context.on_item_leave.call(event);
                } else {
                    content_context.on_item_enter.call(event.clone());
                    if !event.default_prevented() {
                        let item = event.current_target().map(|target| target.unchecked_into::<web_sys::HtmlElement>()).expect("Current target should exist.");
                        // TODO: focus options
                        item.focus().expect("Element should be focused.");
                    }
                }
            })), None)
            on:pointerleave=compose_callbacks(on_pointer_leave, Some(when_mouse(move |event| {
                content_context.on_item_leave.call(event);
            })), None)
            on:focus=compose_callbacks(on_focus, Some(Callback::new(move |_| {
                set_is_focused.set(true);
            })), None)
            on:blur=compose_callbacks(on_focus, Some(Callback::new(move |_| {
                set_is_focused.set(false);
            })), None)
        >
            {children()}
        </Primitive>
    }
}

#[component]
pub fn MenuCheckboxItem() -> impl IntoView {
    view! {}
}

#[component]
pub fn MenuRadioGroup() -> impl IntoView {
    view! {}
}

#[component]
pub fn MenuRadioItem() -> impl IntoView {
    view! {}
}

#[component]
pub fn MenuItemIndicator() -> impl IntoView {
    view! {}
}

#[component]
pub fn MenuSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let mut attrs = attrs.clone();
    attrs.extend([
        ("role", "separator".into_attribute()),
        ("aria-orientation", "horizontal".into_attribute()),
    ]);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attrs=attrs
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

#[component]
pub fn MenuArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <PopperArrow
            as_child=as_child
            node_ref=node_ref
            attrs=attrs
        >
            {children()}
        </PopperArrow>
    }
}

#[component]
pub fn MenuSub() -> impl IntoView {
    view! {}
}

#[component]
pub fn MenuSubTrigger() -> impl IntoView {
    view! {}
}

#[component]
pub fn MenuSubContent() -> impl IntoView {
    view! {}
}

fn get_open_state(open: bool) -> String {
    match open {
        true => "open".into(),
        false => "closed".into(),
    }
}

fn focus_first(candidates: Vec<web_sys::HtmlElement>) {
    let previously_focused_element = document().active_element();
    for candidate in candidates {
        // If focus is already where we want to go, we don't want to keep going through the candidates.
        if previously_focused_element.as_ref() == candidate.dyn_ref::<web_sys::Element>() {
            return;
        }

        candidate.focus().expect("Element should be focused.");
        if document().active_element() != previously_focused_element {
            return;
        }
    }
}

#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

type Polygon = Vec<Point>;

#[derive(Clone, Debug, PartialEq)]
enum Side {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct GraceIntent {
    area: Polygon,
    side: Side,
}

/// Determine if a point is inside of a polygon.
fn is_point_in_polygon(point: Point, polygon: Polygon) -> bool {
    let Point { x, y } = point;
    let mut inside = false;

    let mut i = 0;
    let mut j = polygon.len() - 1;
    while i < polygon.len() {
        let xi = polygon[i].x;
        let yi = polygon[i].y;
        let xj = polygon[j].x;
        let yj = polygon[j].y;

        let intersect = ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }

        j = i;
        i += 1;
    }

    inside
}

fn is_pointer_in_grace_area(event: &PointerEvent, area: Option<Polygon>) -> bool {
    if let Some(area) = area {
        let cursor_pos = Point {
            x: event.client_x() as f64,
            y: event.client_y() as f64,
        };
        is_point_in_polygon(cursor_pos, area)
    } else {
        false
    }
}

fn when_mouse<H: Fn(PointerEvent) + 'static>(handler: H) -> Callback<PointerEvent> {
    Callback::new(move |event: PointerEvent| {
        if event.pointer_type() == "mouse" {
            handler(event);
        }
    })
}
