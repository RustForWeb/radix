// TODO: remove
#![allow(dead_code, unused_variables)]

use leptos::{html::AnyElement, *};
use radix_leptos_direction::{use_direction, Direction};
use radix_leptos_popper::{Popper, PopperAnchor, PopperContent};
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
    children: Children,
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
    children: Children,
) -> impl IntoView {
    // TODO: popper scope
    view! {
        <PopperAnchor attrs=attrs>
            {children()}
        </PopperAnchor>
    }
}

#[component]
pub fn MenuPortal(children: Children) -> impl IntoView {
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
    children: Children,
) -> impl IntoView {
    // TODO
    view! {
        <MenuContentImpl>
            {children()}
        </MenuContentImpl>
    }
}

#[component]
fn MenuRootContentNonModal(
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    // TODO
    view! {
        <MenuContentImpl>
            {children()}
        </MenuContentImpl>
    }
}

#[component]
fn MenuContentImpl(children: Children) -> impl IntoView {
    // TODO
    view! {
        <PopperContent>
            {children()}
        </PopperContent>
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
pub fn MenuArrow() -> impl IntoView {
    view! {}
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
