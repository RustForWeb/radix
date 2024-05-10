// TODO: remove
#![allow(dead_code, unused_variables)]

use leptos::{html::AnyElement, *};
use radix_leptos_direction::{use_direction, Direction};
use radix_leptos_popper::{Popper, PopperAnchor};

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
    let open = Signal::derive(move || open().unwrap_or(false));
    let modal = Signal::derive(move || modal().unwrap_or(true));

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
    // #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    // TODO: popper scope
    view! {
        // {..attributes}
        <PopperAnchor >
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
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let root_context = expect_context::<MenuRootContextValue>();

    view! {
        // TODO: wrapper components
        // TODO: fix this
        // <div>
        //     {move || match (root_context.modal)() {
        //         true => view! {
        //             <MenuRootContentModal {..attributes} class=class>
        //                 {children()}
        //             </MenuRootContentModal>
        //         },
        //         false => view! {
        //             <MenuRootContentNonModal {..attributes} class=class>
        //                 {children()}
        //             </MenuRootContentNonModal>
        //         }
        //     }}
        // </div>
    }
}

#[component]
fn MenuRootContentModal(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    view! {}
}

#[component]
fn MenuRootContentNonModal(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    view! {}
}

#[component]
fn MenuContentImpl() -> impl IntoView {
    view! {}
}

#[component]
pub fn MenuGroup() -> impl IntoView {
    view! {}
}

#[component]
pub fn MenuLabel() -> impl IntoView {
    view! {}
}

#[component]
pub fn MenuItem(
    #[prop(into, optional)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {}
}

#[component]
fn MenuItemImpl() -> impl IntoView {
    view! {}
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
pub fn MenuSeparator() -> impl IntoView {
    view! {}
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
