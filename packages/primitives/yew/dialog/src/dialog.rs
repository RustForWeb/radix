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
    pub as_child: Option<Callback<DialogPortalAsChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

pub type DialogPortalAsChildProps = PortalChildProps;

#[function_component]
pub fn DialogPortal(props: &DialogPortalProps) -> Html {
    let context = use_context::<DialogContextValue>().expect("Dialog context required.");

    let context_value = use_memo(props.force_mount, |force_mount| DialogPortalContextValue {
        force_mount: *force_mount,
    });

    // TODO: reduce cloning of children?
    let mut children = props.children.clone();
    let children_list = children.to_vlist_mut();

    html! {
        <ContextProvider<DialogPortalContextValue> context={(*context_value).clone()}>
            {children_list.iter().map(|child| html! {
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

fn get_state(open: bool) -> String {
    if open { "open" } else { "closed" }.to_owned()
}
