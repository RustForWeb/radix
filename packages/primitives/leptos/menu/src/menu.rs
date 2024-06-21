// TODO: remove
#![allow(dead_code, unused_variables)]

use leptos::{
    ev::{Event, FocusEvent, PointerEvent},
    html::AnyElement,
    *,
};
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_direction::{use_direction, Direction};
use radix_leptos_focus_guards::use_focus_guards;
use radix_leptos_focus_scope::FocusScope;
use radix_leptos_popper::{Popper, PopperAnchor, PopperArrow, PopperContent};
use radix_leptos_primitive::{compose_callbacks, Primitive};

#[derive(Clone)]
struct MenuContextValue {
    open: Signal<bool>,
    content_ref: NodeRef<AnyElement>,
    // TODO: onOpenChange
}

#[derive(Clone)]
struct MenuRootContextValue {
    is_using_keyboard: Signal<bool>,
    dir: Signal<Direction>,
    modal: Signal<bool>,
    // TODO: onClose
}

#[component]
pub fn Menu(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    // TODO: onOpenChange
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let open = Signal::derive(move || open.get().unwrap_or(false));
    let modal = Signal::derive(move || modal.get().unwrap_or(true));

    // TODO: popper scope
    let content_ref = create_node_ref::<AnyElement>();
    let is_using_keyboard = create_rw_signal(false);
    let direction = use_direction(dir);

    let context_value = StoredValue::new(MenuContextValue { open, content_ref });
    let root_context_value = StoredValue::new(MenuRootContextValue {
        is_using_keyboard: is_using_keyboard.into(),
        dir: direction,
        modal,
    });

    create_effect(move |_| {
        // TODO: event handlers
    });

    on_cleanup(move || {
        // TODO: cleanup event handlers
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
    // TODO: popper scope
    view! {
        <PopperAnchor as_child=as_child node_ref=node_ref attrs=attrs>
            {children()}
        </PopperAnchor>
    }
}

#[component]
pub fn MenuPortal(children: ChildrenFn) -> impl IntoView {
    // TODO: portal
    // view! {}
    children()
}

#[derive(Clone)]
struct MenuContentContextValue {
    on_item_enter: Callback<PointerEvent>,
    on_item_leave: Callback<PointerEvent>,
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
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    // TODO
    view! {
        <MenuContentImpl as_child=as_child node_ref=node_ref attrs=attrs>
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
    // TODO
    view! {
        <MenuContentImpl as_child=as_child node_ref=node_ref attrs=attrs>
            {children()}
        </MenuContentImpl>
    }
}

#[component]
fn MenuContentImpl(
    /// Event handler called when auto-focusing on open. Can be prevented.
    #[prop(into, optional)]
    on_open_auto_focus: MaybeProp<Callback<Event>>,
    /// Event handler called when auto-focusing on close. Can be prevented.
    #[prop(into, optional)]
    on_close_auto_focus: MaybeProp<Callback<Event>>,
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
    let attrs = StoredValue::new(attrs);
    let children = StoredValue::new(children);

    let content_ref = create_node_ref::<AnyElement>();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);

    // Make sure the whole tree has focus guards as our `MenuContent` may be
    // the last element in the DOM (beacuse of the `Portal`).
    use_focus_guards();

    let handle_mount_auto_focus = Callback::new(compose_callbacks(
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
    ));

    // TODO
    view! {
        <FocusScope
            as_child=true
            trapped=trap_focus
            on_mount_auto_focus=handle_mount_auto_focus
            on_unmount_auto_focus=on_close_auto_focus
        >
            <PopperContent as_child=as_child node_ref=composed_refs attrs=attrs.get_value()>
                {children.with_value(|children| children())}
            </PopperContent>
        </FocusScope>
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

#[component]
pub fn MenuItem(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <MenuItemImpl as_child=as_child node_ref=node_ref attrs=attrs>
            {children()}
        </MenuItemImpl>
    }
}

#[component]
fn MenuItemImpl(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_pointer_move: MaybeProp<Callback<PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: MaybeProp<Callback<PointerEvent>>,
    #[prop(into, optional)] on_focus: MaybeProp<Callback<FocusEvent>>,
    #[prop(into, optional)] on_blur: MaybeProp<Callback<FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let disabled = Signal::derive(move || disabled.get().unwrap_or(true));

    let content_context = expect_context::<MenuContentContextValue>();
    let (is_focused, set_is_focused) = create_signal(false);

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

    // TODO

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attrs=attrs
            on:pointermove=compose_callbacks(on_pointer_move, Some(Callback::new(move |event| {
                if disabled.get() {
                    content_context.on_item_leave.call(event);
                } else {
                    content_context.on_item_enter.call(event.clone());
                    if !event.default_prevented() {
                        let item = event.current_target().map(|target| target.unchecked_into::<web_sys::HtmlElement>());
                        // TODO: focus options
                        item.focus();
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
    children: ChildrenFn,
) -> impl IntoView {
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
            {children()}
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

fn when_mouse<H: Fn(PointerEvent) + 'static>(handler: H) -> Callback<PointerEvent> {
    Callback::new(move |event: PointerEvent| {
        if event.pointer_type() == "mouse" {
            handler(event);
        }
    })
}
