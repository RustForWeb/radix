// TODO: remove
#![allow(dead_code, unused_variables)]

use std::rc::Rc;

use leptos::{ev::Event, html::AnyElement, *};
use radix_leptos_direction::{use_direction, Direction};
use radix_leptos_focus_scope::FocusScope;
use radix_leptos_popper::{Popper, PopperAnchor, PopperArrow, PopperContent};
use radix_leptos_primitive::Primitive;

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
    let open = Signal::derive(move || open.get().unwrap_or(false));
    let modal = Signal::derive(move || modal.get().unwrap_or(true));

    // TODO: popper scope
    let content_ref = create_node_ref::<AnyElement>();
    let is_using_keyboard = create_rw_signal(false);
    let direction = use_direction(dir);

    let context_value = MenuContextValue { open, content_ref };
    let root_context_value = MenuRootContextValue {
        is_using_keyboard: is_using_keyboard.into(),
        dir: direction,
        modal,
    };

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
                    {children()}
                </Provider>
            </Provider>
        </Popper>
    }
}

#[component]
pub fn MenuAnchor(
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    // TODO: popper scope
    view! {
        <PopperAnchor attrs=attrs>
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

#[component]
pub fn MenuContent(
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
            <MenuRootContentModal attrs=attrs.get_value()>
                {children.with_value(|children| children())}
            </MenuRootContentModal>
        </Show>
    }
}

#[component]
fn MenuRootContentModal(
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    // TODO
    view! {
        <MenuContentImpl attrs=attrs>
            {children()}
        </MenuContentImpl>
    }
}

#[component]
fn MenuRootContentNonModal(
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    // TODO
    view! {
        <MenuContentImpl attrs=attrs>
            {children()}
        </MenuContentImpl>
    }
}

#[component]
fn MenuContentImpl(
    /// Event handler called when auto-focusing on open. Can be prevented.
    #[prop(into, optional)]
    on_open_auto_focus: MaybeProp<Rc<dyn Fn(Event)>>,
    /// Event handler called when auto-focusing on close. Can be prevented.
    #[prop(into, optional)]
    on_close_auto_focus: MaybeProp<Rc<dyn Fn(Event)>>,
    /// Whether scrolling outside the `MenuContent` should be prevented. Defaults to `false`.
    #[prop(into, optional)]
    disable_outside_scroll: MaybeProp<bool>,
    /// Whether focus should be trapped within the `MenuContent`. Defaults to `false`.
    #[prop(into, optional)]
    trap_focus: MaybeProp<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let attrs = StoredValue::new(attrs);
    let children = StoredValue::new(children);

    // TODO: compose with on_open_auto_focus
    // TODO: consider if Rc is need, does fn(Event) work?
    let handle_mount_auto_focus: Rc<dyn Fn(Event)> = Rc::new(move |event: Event| {
        // When opening, explicitly focus the content area only and leave `onEntryFocus` in  control of focusing first item.
        event.prevent_default();
        // TODO: content ref focus
    });

    // TODO
    view! {
        <FocusScope
            as_child=true
            trapped=trap_focus
            on_mount_auto_focus=handle_mount_auto_focus
            on_unmount_auto_focus=on_close_auto_focus
        >
            <PopperContent attrs=attrs.get_value()>
                {children.with_value(|children| children()).into_view()}
            </PopperContent>
        </FocusScope>
    }
}

#[component]
pub fn MenuGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let mut attrs = attrs.clone();
    attrs.extend([("role", "group".into_attribute())]);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            attrs=attrs
        >
            {children()}
        </Primitive>
    }
}

#[component]
pub fn MenuLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Primitive
            element=html::div
            as_child=as_child
            attrs=attrs
        >
            {children()}
        </Primitive>
    }
}

#[component]
pub fn MenuItem(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <MenuItemImpl as_child=as_child attrs=attrs>
            {children()}
        </MenuItemImpl>
    }
}

#[component]
fn MenuItemImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let mut attrs = attrs.clone();
    attrs.extend([("role", "menuitem".into_attribute())]);

    // TODO

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            attrs=attrs
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
            attrs=attrs
        >
            {children()}
        </Primitive>
    }
}

#[component]
pub fn MenuArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <PopperArrow
            as_child=as_child
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
