use radix_yew_dismissable_layer::DismissableLayer;
use radix_yew_focus_guards::use_focus_guards;
use radix_yew_focus_scope::{FocusScope, FocusScopeChildProps};
use radix_yew_id::use_id;
use radix_yew_portal::{Portal, PortalChildProps};
use radix_yew_presence::{Presence, PresenceChildProps};
use radix_yew_primitive::compose_callbacks;
use radix_yew_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};
use yew_style::Style;

#[derive(Clone, PartialEq)]
struct DialogContextValue {
    trigger_ref: NodeRef,
    content_ref: NodeRef,
    content_id: String,
    title_id: String,
    description_id: String,
    open: bool,
    on_open_change: Callback<bool>,
    on_open_toggle: Callback<()>,
    modal: bool,
}

#[derive(PartialEq, Properties)]
pub struct DialogProps {
    #[prop_or_default]
    pub open: Option<bool>,
    #[prop_or_default]
    pub default_open: Option<bool>,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    #[prop_or(true)]
    pub modal: bool,

    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Dialog(props: &DialogProps) -> Html {
    let trigger_ref = use_node_ref();
    let content_ref = use_node_ref();

    let on_open_change = use_callback(props.on_open_change.clone(), |value, on_open_change| {
        if let Some(value) = value {
            on_open_change.emit(value);
        }
    });
    let (open, set_open) = use_controllable_state(UseControllableStateParams {
        prop: props.open,
        default_prop: props.default_open,
        on_change: Some(on_open_change),
    });
    let open = open.unwrap_or(false);

    let handle_open_change = use_callback(set_open.clone(), |value, set_open| {
        set_open.emit(Some(value));
    });
    let handle_open_toggle = use_callback((open, set_open), |_, (open, set_open)| {
        set_open.emit(Some(!open));
    });

    let content_id = use_id(None);
    let title_id = use_id(None);
    let description_id = use_id(None);

    let context_value = use_memo(
        (
            props.modal,
            open,
            handle_open_change,
            handle_open_toggle,
            content_id,
            title_id,
            description_id,
        ),
        |(
            modal,
            open,
            handle_open_change,
            handle_open_toggle,
            content_id,
            title_id,
            description_id,
        )| DialogContextValue {
            trigger_ref,
            content_ref,
            content_id: content_id.clone(),
            title_id: title_id.clone(),
            description_id: description_id.clone(),
            open: *open,
            on_open_change: handle_open_change.clone(),
            on_open_toggle: handle_open_toggle.clone(),
            modal: *modal,
        },
    );

    html! {
        <ContextProvider<DialogContextValue> context={(*context_value).clone()}>
            {props.children.clone()}
        </ContextProvider<DialogContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct DialogTriggerProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `button`
    #[prop_or_default]
    pub disabled: bool,

    // Event handler attributes
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<DialogTriggerChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "button")]
pub struct DialogTriggerChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub aria_controls: String,
    pub aria_expanded: String,
    pub aria_haspopup: String,
    pub class: Option<String>,
    pub data_state: String,
    pub id: Option<String>,
    pub style: Style,

    // Attributes from `button`
    pub disabled: bool,
    pub r#type: String,

    // Event handler attributes
    pub onblur: Callback<FocusEvent>,
    pub onclick: Callback<MouseEvent>,
    pub onfocus: Callback<FocusEvent>,
}

#[function_component]
pub fn DialogTrigger(props: &DialogTriggerProps) -> Html {
    let context = use_context::<DialogContextValue>().expect("Dialog context required.");
    let composed_ref = use_composed_ref(&[props.node_ref.clone(), context.trigger_ref]);

    let onclick = compose_callbacks(
        Some(props.on_click.clone()),
        Some(Callback::from(move |_: MouseEvent| {
            context.on_open_toggle.emit(());
        })),
        None,
    );

    let child_props = DialogTriggerChildProps {
        node_ref: composed_ref,
        attributes: props.attributes.clone(),

        // Global attributes
        aria_controls: context.content_id,
        aria_expanded: if context.open { "true" } else { "false" }.to_owned(),
        aria_haspopup: "dialog".to_owned(),
        class: props.class.clone(),
        data_state: get_state(context.open),
        id: props.id.clone(),
        style: props.style.clone(),

        // Attributes from `button`
        disabled: props.disabled,
        r#type: "button".to_owned(),

        // Event handler attributes
        onblur: props.on_blur.clone(),
        onclick,
        onfocus: props.on_focus.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(Clone, PartialEq)]
struct DialogPortalContextValue {
    force_mount: Option<bool>,
}

#[derive(PartialEq, Properties)]
pub struct DialogPortalProps {
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop_or_default]
    pub force_mount: Option<bool>,

    // Props from `Portal`
    /// Specify a container element to portal the content into.
    #[prop_or_default]
    pub container: Option<web_sys::Element>,
    /// Specify a container element to portal the content into.
    #[prop_or_default]
    pub container_ref: Option<NodeRef>,

    #[prop_or_default]
    pub as_child: Option<Callback<DialogPortalChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

pub type DialogPortalChildProps = PortalChildProps;

#[function_component]
pub fn DialogPortal(props: &DialogPortalProps) -> Html {
    let context = use_context::<DialogContextValue>().expect("Dialog context required.");

    let context_value = use_memo(props.force_mount, |force_mount| DialogPortalContextValue {
        force_mount: *force_mount,
    });

    let mut children = props.children.clone();
    let children_list = children.to_vlist_mut();

    html! {
        <ContextProvider<DialogPortalContextValue> context={(*context_value).clone()}>
            // Reverse iterator, so the child portals render in the same order as in React.
            {children_list.iter().rev().map(|child| html! {
                <Presence
                    present={props.force_mount.unwrap_or_default() || context.open}

                    as_child={Callback::from({
                        let container = props.container.clone();
                        let container_ref = props.container_ref.clone();

                        let as_child = props.as_child.clone();
                        let child = child.clone();

                        move |PresenceChildProps { node_ref }| html! {
                            // TODO: as_child by default?
                            <Portal
                                container={container.clone()}
                                container_ref={container_ref.clone()}

                                node_ref={node_ref}
                                as_child={as_child.clone()}
                            >
                                // TODO: pass node_ref to child
                                {child.clone()}
                            </Portal>
                        }
                    })}
                />
            }).collect::<Html>()}
        </ContextProvider<DialogPortalContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct DialogOverlayProps {
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop_or_default]
    pub force_mount: Option<bool>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<DialogOverlayChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

pub type DialogOverlayChildProps = DialogOverlayImplChildProps;

#[function_component]
pub fn DialogOverlay(props: &DialogOverlayProps) -> Html {
    let portal_context =
        use_context::<DialogPortalContextValue>().expect("Dialog portal context required.");
    let context = use_context::<DialogContextValue>().expect("Dialog context required.");

    let force_mount = props.force_mount.or(portal_context.force_mount);

    html! {
        if context.modal {
            <Presence
                present={force_mount.unwrap_or_default() || context.open}

                node_ref={props.node_ref.clone()}
                as_child={Callback::from({
                    let class = props.class.clone();
                    let id = props.id.clone();
                    let style = props.style.clone();

                    let attributes = props.attributes.clone();
                    let as_child = props.as_child.clone();
                    let children = props.children.clone();

                    move |PresenceChildProps { node_ref }| html! {
                        <DialogOverlayImpl
                            class={class.clone()}
                            id={id.clone()}
                            style={style.clone()}

                            node_ref={node_ref.clone()}
                            attributes={attributes.clone()}
                            as_child={as_child.clone()}
                        >
                            {children.clone()}
                        </DialogOverlayImpl>
                    }
                })}
            />
        }
    }
}

#[derive(PartialEq, Properties)]
struct DialogOverlayImplProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<DialogOverlayImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct DialogOverlayImplChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub data_state: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
fn DialogOverlayImpl(props: &DialogOverlayImplProps) -> Html {
    let context = use_context::<DialogContextValue>().expect("Dialog context required.");

    let child_props = DialogOverlayImplChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        data_state: get_state(context.open),
        id: props.id.clone(),
        style: props.style.clone().with_defaults([
            // We re-enable pointer-events prevented by `DialogContent` to allow scrolling the overlay.
            ("pointer-events", "auto"),
        ]),
    };

    html! {
        // Make sure `Content` is scrollable even when it doesn't live inside `RemoveScroll`
        // i.e. when `Overlay` and `Content` are siblings.
        // TODO: RemoveScroll

        if let Some(as_child) = props.as_child.as_ref() {
            {as_child.emit(child_props)}
        } else {
            {child_props.render(props.children.clone())}
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct DialogContentProps {
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop_or_default]
    pub force_mount: Option<bool>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<DialogContentChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

pub type DialogContentChildProps = DialogContentImplChildProps;

#[function_component]
pub fn DialogContent(props: &DialogContentProps) -> Html {
    let portal_context =
        use_context::<DialogPortalContextValue>().expect("Dialog portal context required.");
    let context = use_context::<DialogContextValue>().expect("Dialog context required.");

    let force_mount = props.force_mount.or(portal_context.force_mount);

    html! {
        if context.modal {
            <Presence
                present={force_mount.unwrap_or_default() || context.open}

                node_ref={props.node_ref.clone()}
                as_child={Callback::from({
                    let class = props.class.clone();
                    let id = props.id.clone();
                    let style = props.style.clone();

                    let attributes = props.attributes.clone();
                    let as_child = props.as_child.clone();
                    let children = props.children.clone();

                    move |PresenceChildProps { node_ref }| html! {
                        if context.modal {
                            <DialogContentModal
                                class={class.clone()}
                                id={id.clone()}
                                style={style.clone()}

                                node_ref={node_ref.clone()}
                                attributes={attributes.clone()}
                                as_child={as_child.clone()}
                            >
                                {children.clone()}
                            </DialogContentModal>
                        } else {
                            <DialogContentNonModal
                                class={class.clone()}
                                id={id.clone()}
                                style={style.clone()}

                                node_ref={node_ref.clone()}
                                attributes={attributes.clone()}
                                as_child={as_child.clone()}
                            >
                                {children.clone()}
                            </DialogContentNonModal>
                        }
                    }
                })}
            />
        }
    }
}

#[derive(PartialEq, Properties)]
struct DialogContentModalProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<DialogContentImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn DialogContentModal(props: &DialogContentModalProps) -> Html {
    let context = use_context::<DialogContextValue>().expect("Dialog context required.");
    let content_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[
        props.node_ref.clone(),
        context.content_ref,
        content_ref.clone(),
    ]);

    // TODO: effect

    html! {
        <DialogContentImpl
            // We make sure focus isn't trapped once `DialogContent` has been closed (closed !== unmounted when animating out).
            trap_focus={context.open}

            // TODO: props

            class={props.class.clone()}
            id={props.id.clone()}
            style={props.style.clone()}

            node_ref={composed_refs}
            attributes={props.attributes.clone()}
            as_child={props.as_child.clone()}
        >
            {props.children.clone()}
        </DialogContentImpl>
    }
}

#[derive(PartialEq, Properties)]
struct DialogContentNonModalProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<DialogContentImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn DialogContentNonModal(props: &DialogContentNonModalProps) -> Html {
    let _context = use_context::<DialogContextValue>().expect("Dialog context required.");

    html! {
        <DialogContentImpl
            trap_focus={false}

            // TODO: props

            class={props.class.clone()}
            id={props.id.clone()}
            style={props.style.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
            as_child={props.as_child.clone()}
        >
            {props.children.clone()}
        </DialogContentImpl>
    }
}

#[derive(PartialEq, Properties)]
struct DialogContentImplProps {
    ///  When `true`, focus cannot escape the `Content` via keyboard, pointer, or a programmatic focus. Defaults to `false`.
    #[prop_or(false)]
    pub trap_focus: bool,
    /// Event handler called when auto-focusing on open. Can be prevented.
    #[prop_or_default]
    pub on_open_auto_focus: Callback<Event>,
    /// Event handler called when auto-focusing on close. Can be prevented.
    #[prop_or_default]
    pub on_close_auto_focus: Callback<Event>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<DialogContentImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct DialogContentImplChildProps {}

#[function_component]
fn DialogContentImpl(props: &DialogContentImplProps) -> Html {
    let context = use_context::<DialogContextValue>().expect("Dialog context required.");
    let content_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), content_ref.clone()]);

    let handle_dismiss = use_callback(context.on_open_change, |_, on_open_change| {
        on_open_change.emit(false);
    });

    // Make sure the whole tree has focus guards as our `Dialog` will be
    // the last element in the DOM (because of the `Portal`).
    use_focus_guards();

    html! {
        <>
            <FocusScope
                r#loop=true
                trapped={props.trap_focus}
                on_mount_auto_focus={props.on_open_auto_focus.clone()}
                on_unmount_auto_focus={props.on_close_auto_focus.clone()}

                class={props.class.clone()}
                id={props.id.clone()}
                style={props.style.clone()}

                node_ref={composed_refs}
                attributes={props.attributes.clone()}
                as_child={Callback::from({
                    let children = props.children.clone();
                    // let as_child = props.as_child.clone();

                    move |FocusScopeChildProps {
                        node_ref,
                        attributes,

                        class,
                        id,
                        style,
                        tabindex,

                        onkeydown
                    }| html! {
                        // TODO: Pass attributes as DialogContentImplChildProps with support for DismissableLayerChildProps.
                        <DismissableLayer
                            on_dismiss={handle_dismiss.clone()}

                            class={class}
                            id={id.unwrap_or(context.content_id.clone())}
                            style={style}

                            on_key_down={onkeydown}

                            node_ref={node_ref}
                            attributes={attributes.with_defaults([
                                ("aria-describedby", context.description_id.clone()),
                                ("aria-labelledby", context.title_id.clone()),
                                ("data-state", get_state(context.open)),
                                ("role", "dialog".to_owned()),
                                ("tabindex", tabindex),
                            ])}
                            // as_child={as_child.clone()}
                        >
                            {children.clone()}
                        </DismissableLayer>
                    }
                })}
            />

            // TODO: TitleWarning, DescriptionWarning
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct DialogTitleProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<DialogTitleChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "h2")]
pub struct DialogTitleChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: String,
    pub style: Style,
}

#[function_component]
pub fn DialogTitle(props: &DialogTitleProps) -> Html {
    let context = use_context::<DialogContextValue>().expect("Dialog context required.");

    let child_props = DialogTitleChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        id: props.id.clone().unwrap_or(context.title_id),
        style: props.style.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(PartialEq, Properties)]
pub struct DialogDescriptionProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<DialogDescriptionChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "p")]
pub struct DialogDescriptionChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: String,
    pub style: Style,
}

#[function_component]
pub fn DialogDescription(props: &DialogDescriptionProps) -> Html {
    let context = use_context::<DialogContextValue>().expect("Dialog context required.");

    let child_props = DialogDescriptionChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        id: props.id.clone().unwrap_or(context.description_id),
        style: props.style.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(PartialEq, Properties)]
pub struct DialogCloseProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `button`
    #[prop_or_default]
    pub disabled: bool,

    // Event handler attributes
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<DialogCloseChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "button")]
pub struct DialogCloseChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: String,
    pub style: Style,

    // Attributes from `button`
    pub disabled: bool,
    pub r#type: String,

    // Event handler attributes
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn DialogClose(props: &DialogCloseProps) -> Html {
    let context = use_context::<DialogContextValue>().expect("Dialog context required.");

    let onclick = compose_callbacks(
        Some(props.on_click.clone()),
        Some(Callback::from(move |_| {
            context.on_open_change.emit(false);
        })),
        None,
    );

    let child_props = DialogCloseChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        id: props.id.clone().unwrap_or(context.description_id),
        style: props.style.clone(),

        // Attributes from `button`
        disabled: props.disabled,
        r#type: "button".to_owned(),

        // Event handler attributes
        onclick,
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

fn get_state(open: bool) -> String {
    if open { "open" } else { "closed" }.to_owned()
}
