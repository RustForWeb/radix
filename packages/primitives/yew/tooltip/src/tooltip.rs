use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt::{self, Display},
    rc::Rc,
};

use radix_yew_id::use_id;
pub use radix_yew_popper::{Align, Padding, Side, Sticky, UpdatePositionStrategy};
use radix_yew_popper::{
    Popper, PopperAnchor, PopperAnchorChildProps, PopperArrow, PopperArrowChildProps,
    PopperContent, PopperContentChildProps,
};
use radix_yew_portal::{Portal, PortalChildProps};
use radix_yew_presence::{Presence, PresenceChildProps};
use radix_yew_primitive::compose_callbacks;
use radix_yew_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use radix_yew_visually_hidden::VisuallyHidden;
use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    window, CustomEvent,
};
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};
use yew_style::Style;

const DEFAULT_DELAY_DURATION: i32 = 700;
const TOOLTIP_OPEN: &str = "tooltip.open";

#[derive(Clone, PartialEq)]
struct TooltipProviderContextValue {
    is_open_delayed: bool,
    delay_duration: i32,
    on_open: Callback<()>,
    on_close: Callback<()>,
    on_pointer_in_transit_change: Callback<bool>,
    is_pointer_in_transit_ref: Rc<RefCell<bool>>,
    disable_hoverable_content: bool,
}

#[derive(PartialEq, Properties)]
pub struct TooltipProviderProps {
    /// The duration from when the pointer enters the trigger until the tooltip gets opened. Defaults to `700`.
    #[prop_or(DEFAULT_DELAY_DURATION)]
    pub delay_duration: i32,
    /// How much time a user has to enter another trigger without incurring a delay again. Defaults to `300`.
    #[prop_or(300)]
    pub skip_delay_duration: i32,
    /// When `true`, trying to hover the content will result in the tooltip closing as the pointer leaves the trigger. Defaults to `false`.
    #[prop_or(false)]
    pub disable_hoverable_content: bool,

    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TooltipProvider(props: &TooltipProviderProps) -> Html {
    let is_open_delayed = use_state_eq(|| true);
    let is_pointer_in_transit_ref = use_mut_ref(|| false);
    let skip_delay_timer_ref = use_mut_ref(|| 0);

    let closure: Rc<Closure<dyn Fn()>> = use_memo((), {
        let is_open_delayed = is_open_delayed.clone();

        move |_| {
            Closure::new(move || {
                is_open_delayed.set(true);
            })
        }
    });

    use_effect_with((), {
        let skip_delay_timer_ref = skip_delay_timer_ref.clone();
        let closure = closure.clone();

        move |_| {
            move || {
                let skip_delay_timer = *skip_delay_timer_ref.borrow();
                window()
                    .expect("Window should exist.")
                    .clear_timeout_with_handle(skip_delay_timer);

                // Move closure to prevent it from being dropped too early.
                drop(closure);
            }
        }
    });

    let on_open = use_callback((), {
        let is_open_delayed = is_open_delayed.clone();
        let skip_delay_timer_ref = skip_delay_timer_ref.clone();

        move |_, _| {
            let skip_delay_timer = *skip_delay_timer_ref.borrow();
            window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(skip_delay_timer);

            is_open_delayed.set(false);
        }
    });

    let on_close = use_callback(props.skip_delay_duration, {
        let skip_delay_timer_ref = skip_delay_timer_ref.clone();

        move |_, skip_delay_duration| {
            let window = window().expect("Window should exist.");

            let skip_delay_timer = *skip_delay_timer_ref.borrow();
            window.clear_timeout_with_handle(skip_delay_timer);

            *skip_delay_timer_ref.borrow_mut() = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    (*closure).as_ref().unchecked_ref(),
                    *skip_delay_duration,
                )
                .expect("Timeout should be started.");
        }
    });

    let on_pointer_in_transit_change = use_callback((), {
        let is_pointer_in_transit_ref = is_pointer_in_transit_ref.clone();

        move |in_transit, _| {
            *is_pointer_in_transit_ref.borrow_mut() = in_transit;
        }
    });

    let context_value = use_memo(
        (
            props.delay_duration,
            props.disable_hoverable_content,
            is_open_delayed,
            on_open,
            on_close,
            on_pointer_in_transit_change,
        ),
        |(
            delay_duration,
            disable_hoverable_content,
            is_open_delayed,
            on_open,
            on_close,
            on_pointer_in_transit_change,
        )| {
            TooltipProviderContextValue {
                is_open_delayed: **is_open_delayed,
                delay_duration: *delay_duration,
                on_open: on_open.clone(),
                on_close: on_close.clone(),
                on_pointer_in_transit_change: on_pointer_in_transit_change.clone(),
                is_pointer_in_transit_ref,
                disable_hoverable_content: *disable_hoverable_content,
            }
        },
    );

    html! {
        <ContextProvider<TooltipProviderContextValue> context={(*context_value).clone()}>
            {props.children.clone()}
        </ContextProvider<TooltipProviderContextValue>>
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum StateAttribute {
    Closed,
    DelayedOpen,
    InstantOpen,
}

impl Display for StateAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StateAttribute::Closed => "closed",
                StateAttribute::DelayedOpen => "delayed-open",
                StateAttribute::InstantOpen => "instant-open",
            }
        )
    }
}

#[derive(Clone, PartialEq)]
struct TooltipContextValue {
    content_id: String,
    open: bool,
    state_attribute: StateAttribute,
    trigger_ref: NodeRef,
    on_trigger_enter: Callback<()>,
    on_trigger_leave: Callback<()>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    disable_hoverable_content: bool,
}

#[derive(PartialEq, Properties)]
pub struct TooltipProps {
    #[prop_or_default]
    pub open: Option<bool>,
    #[prop_or_default]
    pub default_open: Option<bool>,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    /// The duration from when the pointer enters the trigger until the tooltip gets opened. Defaults to `700`.
    #[prop_or_default]
    pub delay_duration: Option<i32>,
    /// When `true`, trying to hover the content will result in the tooltip closing as the pointer leaves the trigger. Defaults to `false`.
    #[prop_or_default]
    pub disable_hoverable_content: Option<bool>,

    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let provider_context =
        use_context::<TooltipProviderContextValue>().expect("Tooltip provider context required.");
    let trigger_ref = use_node_ref();
    let content_id = use_id(None);
    let open_timer_ref = use_mut_ref(|| 0);
    let disable_hoverable_content = props
        .disable_hoverable_content
        .unwrap_or(provider_context.disable_hoverable_content);
    let delay_duration = props
        .delay_duration
        .unwrap_or(provider_context.delay_duration);
    let was_open_delayed_ref = use_mut_ref(|| false);

    let on_open_change = use_callback(
        (
            props.on_open_change.clone(),
            provider_context.on_open,
            provider_context.on_close,
        ),
        |value: Option<bool>,
         (on_open_change, provider_context_on_open, provider_context_on_close)| {
            if let Some(value) = value {
                if value {
                    provider_context_on_open.emit(());

                    let event = CustomEvent::new(TOOLTIP_OPEN)
                        .expect("Tooltip open event should be instantiated.");
                    window()
                        .expect("Window should exist.")
                        .document()
                        .expect("Document should exist.")
                        .dispatch_event(&event)
                        .expect("Tooltip open event should be dispatched.");
                } else {
                    provider_context_on_close.emit(());
                }

                on_open_change.emit(value);
            }
        },
    );
    let (open, set_open) = use_controllable_state(UseControllableStateParams {
        prop: props.open,
        default_prop: props.default_open,
        on_change: Some(on_open_change),
    });
    let open = open.unwrap_or(false);

    let state_attribute = use_memo(open, |open| {
        if *open {
            let was_open_delayed = *was_open_delayed_ref.borrow();
            if was_open_delayed {
                StateAttribute::DelayedOpen
            } else {
                StateAttribute::InstantOpen
            }
        } else {
            StateAttribute::Closed
        }
    });

    let handle_open = use_callback(set_open.clone(), {
        let open_timer_ref = open_timer_ref.clone();
        let was_open_delayed_ref = was_open_delayed_ref.clone();

        move |_, set_open| {
            let open_timer = *open_timer_ref.borrow();
            window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(open_timer);
            *open_timer_ref.borrow_mut() = 0;

            *was_open_delayed_ref.borrow_mut() = false;

            set_open.emit(Some(true));
        }
    });

    let handle_close = use_callback(set_open.clone(), {
        let open_timer_ref = open_timer_ref.clone();

        move |_, set_open| {
            let open_timer = *open_timer_ref.borrow();
            window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(open_timer);
            *open_timer_ref.borrow_mut() = 0;

            set_open.emit(Some(false));
        }
    });

    let closure: Rc<Closure<dyn Fn()>> = use_memo((), {
        let set_open = set_open.clone();
        let open_timer_ref = open_timer_ref.clone();

        move |_| {
            Closure::new(move || {
                *was_open_delayed_ref.borrow_mut() = true;
                set_open.emit(Some(true));
                *open_timer_ref.borrow_mut() = 0;
            })
        }
    });

    let handle_delayed_open = use_callback(delay_duration, {
        let open_timer_ref = open_timer_ref.clone();
        let closure = closure.clone();

        move |_, delay_duration| {
            let window = window().expect("Window should exist.");

            let open_timer = *open_timer_ref.borrow();
            window.clear_timeout_with_handle(open_timer);

            *open_timer_ref.borrow_mut() = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    (*closure).as_ref().unchecked_ref(),
                    *delay_duration,
                )
                .expect("Timeout should be started");
        }
    });

    use_effect_with((), {
        let open_timer_ref = open_timer_ref.clone();

        move |_| {
            move || {
                let open_timer = *open_timer_ref.borrow();
                if open_timer != 0 {
                    window()
                        .expect("Window should exist.")
                        .clear_timeout_with_handle(open_timer);
                    *open_timer_ref.borrow_mut() = 0;
                }

                // Move closure to prevent it from being dropped too early.
                drop(closure);
            }
        }
    });

    let handle_trigger_enter = use_callback(
        (
            provider_context.is_open_delayed,
            handle_delayed_open,
            handle_open.clone(),
        ),
        |_, (is_open_delayed, handle_delayed_open, handle_open)| {
            if *is_open_delayed {
                handle_delayed_open.emit(());
            } else {
                handle_open.emit(());
            }
        },
    );
    let handle_trigger_leave = use_callback(
        (disable_hoverable_content, handle_close.clone()),
        move |_, (disable_hoverable_content, handle_close)| {
            if *disable_hoverable_content {
                handle_close.emit(());
            } else {
                // Clear the timer in case the pointer leaves the trigger before the tooltip is opened.
                let open_timer = *open_timer_ref.borrow();
                window()
                    .expect("Window should exist.")
                    .clear_timeout_with_handle(open_timer);
                *open_timer_ref.borrow_mut() = 0;
            }
        },
    );

    let context_value = use_memo(
        (
            content_id,
            disable_hoverable_content,
            open,
            state_attribute,
            handle_open,
            handle_close,
            handle_trigger_enter,
            handle_trigger_leave,
        ),
        |(
            content_id,
            disable_hoverable_content,
            open,
            state_attribute,
            handle_open,
            handle_close,
            handle_trigger_enter,
            handle_trigger_leave,
        )| TooltipContextValue {
            content_id: content_id.clone(),
            open: *open,
            state_attribute: **state_attribute,
            trigger_ref,
            on_trigger_enter: handle_trigger_enter.clone(),
            on_trigger_leave: handle_trigger_leave.clone(),
            on_open: handle_open.clone(),
            on_close: handle_close.clone(),
            disable_hoverable_content: *disable_hoverable_content,
        },
    );

    html! {
        <Popper>
            <ContextProvider<TooltipContextValue> context={(*context_value).clone()}>
                {props.children.clone()}
            </ContextProvider<TooltipContextValue>>
        </Popper>
    }
}

#[derive(PartialEq, Properties)]
pub struct TooltipTriggerProps {
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
    pub on_pointer_down: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_move: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_leave: Callback<PointerEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<TooltipTriggerChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "button")]
pub struct TooltipTriggerChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub aria_describedby: Option<String>,
    pub class: Option<String>,
    pub data_state: String,
    pub id: Option<String>,
    pub style: Style,

    // Event handler attributes
    pub onblur: Callback<FocusEvent>,
    pub onclick: Callback<MouseEvent>,
    pub onfocus: Callback<FocusEvent>,
    pub onpointerdown: Callback<PointerEvent>,
    pub onpointerleave: Callback<PointerEvent>,
    pub onpointermove: Callback<PointerEvent>,
}

#[function_component]
pub fn TooltipTrigger(props: &TooltipTriggerProps) -> Html {
    let context = use_context::<TooltipContextValue>().expect("Tooltip context required.");
    let provider_context =
        use_context::<TooltipProviderContextValue>().expect("Tooltip provider context required.");
    let trigger_ref = use_node_ref();
    let composed_ref = use_composed_ref(&[
        props.node_ref.clone(),
        trigger_ref.clone(),
        context.trigger_ref,
    ]);
    let is_pointer_down_ref = use_mut_ref(|| false);
    let has_pointer_move_opened_ref = use_mut_ref(|| false);

    let handle_pointer_up: Rc<Closure<dyn Fn(PointerEvent)>> = use_memo((), {
        let is_pointer_down_ref = is_pointer_down_ref.clone();

        move |_| {
            Closure::new(move |_: PointerEvent| {
                *is_pointer_down_ref.borrow_mut() = false;
            })
        }
    });

    use_effect_with((), {
        let handle_pointer_up = handle_pointer_up.clone();

        move |_| {
            move || {
                window()
                    .expect("Window should exist.")
                    .document()
                    .expect("Document should exist.")
                    .add_event_listener_with_callback(
                        "pointerup",
                        (*handle_pointer_up).as_ref().unchecked_ref(),
                    )
                    .expect("Pointer up event handler should be removed.");
            }
        }
    });

    // TOOD: effect

    html! {
        <PopperAnchor
            node_ref={composed_ref}
            attributes={props.attributes.clone()}
            as_child={Callback::from({
                let id = props.id.clone();
                let class = props.class.clone();
                let style = props.style.clone();

                let on_blur = props.on_blur.clone();
                let on_click = props.on_click.clone();
                let on_focus = props.on_focus.clone();
                let on_pointer_down = props.on_pointer_down.clone();
                let on_pointer_leave = props.on_pointer_leave.clone();
                let on_pointer_move = props.on_pointer_move.clone();

                let content_id = context.content_id.clone();
                let on_trigger_enter = context.on_trigger_enter.clone();
                let on_trigger_leave = context.on_trigger_leave.clone();
                let on_open = context.on_open.clone();
                let on_close = context.on_close.clone();
                let is_pointer_in_transit_ref = provider_context.is_pointer_in_transit_ref.clone();
                let is_pointer_down_ref = is_pointer_down_ref.clone();
                let handle_pointer_up = handle_pointer_up.clone();

                let as_child = props.as_child.clone();
                let children = props.children.clone();

                move |PopperAnchorChildProps { node_ref, attributes, .. }| {
                    let child_props = TooltipTriggerChildProps {
                        node_ref,
                        attributes,

                        // Global attributes
                        aria_describedby: context.open.then_some(content_id.clone()),
                        class: class.clone(),
                        data_state: context.state_attribute.to_string(),
                        id: id.clone(),
                        style: style.clone(),

                        // Attributes from `button`
                        // We purposefully avoid adding `type="button"` here because tooltip triggers are also
                        // commonly anchors and the anchor `type` attribute signifies MIME type.

                        // Event handler attributes
                        onblur: compose_callbacks(Some(on_blur.clone()), Some(Callback::from({
                            let on_close = on_close.clone();

                            move |_: FocusEvent| {
                                on_close.emit(());
                            }
                        })), None),
                        onclick: compose_callbacks(Some(on_click.clone()), Some(Callback::from({
                            let on_close = on_close.clone();

                            move |_: MouseEvent| {
                                on_close.emit(());
                            }
                        })), None),
                        onfocus: compose_callbacks(Some(on_focus.clone()), Some(Callback::from({
                            let on_open = on_open.clone();
                            let is_pointer_down_ref = is_pointer_down_ref.clone();

                            move |_: FocusEvent| {
                                if !*is_pointer_down_ref.borrow() {
                                    on_open.emit(());
                                }
                            }
                        })), None),
                        onpointerdown: compose_callbacks(Some(on_pointer_down.clone()), Some(Callback::from({
                            let is_pointer_down_ref = is_pointer_down_ref.clone();
                            let handle_pointer_up = handle_pointer_up.clone();

                            move |_: PointerEvent| {
                                *is_pointer_down_ref.borrow_mut() = true;

                                let options = web_sys::AddEventListenerOptions::new();
                                options.set_once(true);

                                window()
                                    .expect("Window should exist.")
                                    .document()
                                    .expect("Document should exist.")
                                    .add_event_listener_with_callback_and_add_event_listener_options(
                                        "pointerup",
                                        (*handle_pointer_up).as_ref().unchecked_ref(),
                                        &options
                                    )
                                    .expect("Pointer up event handler should be added.");
                            }
                        })), None),
                        onpointerleave: compose_callbacks(Some(on_pointer_leave.clone()), Some(Callback::from({
                            let on_trigger_leave = on_trigger_leave.clone();
                            let has_pointer_move_opened_ref = has_pointer_move_opened_ref.clone();

                            move |_: PointerEvent| {
                                on_trigger_leave.emit(());
                                *has_pointer_move_opened_ref.borrow_mut() = false;
                            }
                        })), None),
                        onpointermove: compose_callbacks(Some(on_pointer_move.clone()), Some(Callback::from({
                            let on_trigger_enter = on_trigger_enter.clone();
                            let has_pointer_move_opened_ref = has_pointer_move_opened_ref.clone();
                            let is_pointer_in_transit_ref = is_pointer_in_transit_ref.clone();

                            move |event: PointerEvent| {
                                if event.pointer_type() == "touch" {
                                    return;
                                }

                                if !*has_pointer_move_opened_ref.borrow() && !*is_pointer_in_transit_ref.borrow() {
                                    on_trigger_enter.emit(());
                                    *has_pointer_move_opened_ref.borrow_mut() = true;
                                }
                            }
                        })), None),
                    };

                    if let Some(as_child) = as_child.as_ref() {
                        as_child.emit(child_props)
                    } else {
                        child_props.render(children.clone())
                    }
                }
            })}
        />
    }
}

#[derive(Clone, PartialEq)]
struct TooltipPortalContextValue {
    force_mount: Option<bool>,
}

#[derive(PartialEq, Properties)]
pub struct TooltipPortalProps {
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
    pub as_child: Option<Callback<TooltipPortalAsChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

pub type TooltipPortalAsChildProps = PortalChildProps;

#[function_component]
pub fn TooltipPortal(props: &TooltipPortalProps) -> Html {
    let context = use_context::<TooltipContextValue>().expect("Tooltip context required.");

    let context_value = use_memo(props.force_mount, |force_mount| TooltipPortalContextValue {
        force_mount: *force_mount,
    });

    html! {
        <ContextProvider<TooltipPortalContextValue> context={(*context_value).clone()}>
            <Presence
                present={props.force_mount.unwrap_or_default() || context.open}

                as_child={Callback::from({
                    let container = props.container.clone();
                    let container_ref = props.container_ref.clone();

                    let as_child = props.as_child.clone();
                    let children = props.children.clone();

                    move |PresenceChildProps { node_ref }| html! {
                        // TODO: as_child by default?
                        <Portal
                            container={container.clone()}
                            container_ref={container_ref.clone()}

                            node_ref={node_ref}
                            as_child={as_child.clone()}
                        >
                            // TODO: pass node_ref to children
                            {children.clone()}
                        </Portal>
                    }
                })}
            />
        </ContextProvider<TooltipPortalContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct TooltipContentProps {
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop_or_default]
    pub force_mount: Option<bool>,

    // Props from `DismissableLayer`
    // TODO: on_escape_key_down, on_pointer_down_outside

    // Props from `PopperContent`
    #[prop_or(Side::Top)]
    pub side: Side,
    #[prop_or(0.0)]
    pub side_offset: f64,
    #[prop_or(Align::Center)]
    pub align: Align,
    #[prop_or(0.0)]
    pub align_offset: f64,
    #[prop_or(0.0)]
    pub arrow_padding: f64,
    #[prop_or(true)]
    pub avoid_collisions: bool,
    #[prop_or_default]
    pub collision_boundary: Vec<web_sys::Element>,
    #[prop_or(Padding::All(0.0))]
    pub collision_padding: Padding,
    #[prop_or(Sticky::Partial)]
    pub sticky: Sticky,
    #[prop_or(false)]
    pub hide_when_detached: bool,
    #[prop_or(UpdatePositionStrategy::Optimized)]
    pub update_position_strategy: UpdatePositionStrategy,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub dir: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub role: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<TooltipContentChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

pub type TooltipContentChildProps = TooltipContentImplChildProps;

#[function_component]
pub fn TooltipContent(props: &TooltipContentProps) -> Html {
    let portal_context =
        use_context::<TooltipPortalContextValue>().expect("Tooltip portal context required.");
    let context = use_context::<TooltipContextValue>().expect("Tooltip context required.");

    let force_mount = props.force_mount.or(portal_context.force_mount);

    html! {
        <Presence
            present={force_mount.unwrap_or_default() || context.open}

            node_ref={props.node_ref.clone()}
            as_child={Callback::from({
                let side = props.side;
                let side_offset = props.side_offset;
                let align = props.align;
                let align_offset = props.align_offset;
                let arrow_padding = props.arrow_padding;
                let avoid_collisions = props.avoid_collisions;
                let collision_boundary = props.collision_boundary.clone();
                let collision_padding = props.collision_padding.clone();
                let sticky = props.sticky;
                let hide_when_detached = props.hide_when_detached;
                let update_position_strategy = props.update_position_strategy;

                let dir = props.dir.clone();
                let class = props.class.clone();
                let id = props.id.clone();
                let role = props.role.clone();
                let style = props.style.clone();

                let attributes = props.attributes.clone();
                let as_child = props.as_child.clone();
                let children = props.children.clone();

                move |PresenceChildProps { node_ref }| html! {
                    if context.disable_hoverable_content {
                        <TooltipContentImpl
                            side={side}
                            side_offset={side_offset}
                            align={align}
                            align_offset={align_offset}
                            arrow_padding={arrow_padding}
                            avoid_collisions={avoid_collisions}
                            collision_boundary={collision_boundary.clone()}
                            collision_padding={collision_padding.clone()}
                            sticky={sticky}
                            hide_when_detached={hide_when_detached}
                            update_position_strategy={update_position_strategy}

                            dir={dir.clone()}
                            class={class.clone()}
                            id={id.clone()}
                            role={role.clone()}
                            style={style.clone()}

                            node_ref={node_ref.clone()}
                            attributes={attributes.clone()}
                            as_child={as_child.clone()}
                        >
                            {children.clone()}
                        </TooltipContentImpl>
                    } else {
                        <TooltipContentHoverable
                            side={side}
                            side_offset={side_offset}
                            align={align}
                            align_offset={align_offset}
                            arrow_padding={arrow_padding}
                            avoid_collisions={avoid_collisions}
                            collision_boundary={collision_boundary.clone()}
                            collision_padding={collision_padding.clone()}
                            sticky={sticky}
                            hide_when_detached={hide_when_detached}
                            update_position_strategy={update_position_strategy}

                            dir={dir.clone()}
                            class={class.clone()}
                            id={id.clone()}
                            role={role.clone()}
                            style={style.clone()}

                            node_ref={node_ref.clone()}
                            attributes={attributes.clone()}
                            as_child={as_child.clone()}
                        >
                            {children.clone()}
                        </TooltipContentHoverable>
                    }
                }
            })}
        />
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

type Polygon = Vec<Point>;

#[derive(PartialEq, Properties)]
struct TooltipContentHoverableProps {
    // Props from `DismissableLayer`
    // TODO: on_escape_key_down, on_pointer_down_outside

    // Props from `PopperContent`
    #[prop_or(Side::Bottom)]
    pub side: Side,
    #[prop_or(0.0)]
    pub side_offset: f64,
    #[prop_or(Align::Center)]
    pub align: Align,
    #[prop_or(0.0)]
    pub align_offset: f64,
    #[prop_or(0.0)]
    pub arrow_padding: f64,
    #[prop_or(true)]
    pub avoid_collisions: bool,
    #[prop_or_default]
    pub collision_boundary: Vec<web_sys::Element>,
    #[prop_or(Padding::All(0.0))]
    pub collision_padding: Padding,
    #[prop_or(Sticky::Partial)]
    pub sticky: Sticky,
    #[prop_or(false)]
    pub hide_when_detached: bool,
    #[prop_or(UpdatePositionStrategy::Optimized)]
    pub update_position_strategy: UpdatePositionStrategy,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub dir: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub role: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<TooltipContentImplChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn TooltipContentHoverable(props: &TooltipContentHoverableProps) -> Html {
    let context = use_context::<TooltipContextValue>().expect("Tooltip context required.");
    let provider_context =
        use_context::<TooltipProviderContextValue>().expect("Tooltip provider context required.");
    let content_ref = use_node_ref();
    let composed_ref = use_composed_ref(&[props.node_ref.clone(), content_ref.clone()]);
    let pointer_grace_area = use_state_eq(|| None::<Polygon>);

    let handle_remove_grace_area =
        use_callback(provider_context.on_pointer_in_transit_change.clone(), {
            let pointer_grace_area = pointer_grace_area.clone();

            move |_, on_pointer_in_transit_change| {
                pointer_grace_area.set(None);
                on_pointer_in_transit_change.emit(false);
            }
        });

    let handle_create_grace_area = use_callback(provider_context.on_pointer_in_transit_change, {
        let pointer_grace_area = pointer_grace_area.clone();
        move |(event, hover_target, current_target): (
            PointerEvent,
            web_sys::HtmlElement,
            web_sys::HtmlElement,
        ),
              on_pointer_in_transit_change| {
            // Yew messes up `current_target`, see https://yew.rs/docs/concepts/html/events#event-delegation.
            //
            // let current_target = event
            //     .current_target()
            //     .expect("Current target should exist.")
            //     .unchecked_into::<web_sys::Element>();

            let exit_point = Point {
                x: event.client_x() as f64,
                y: event.client_y() as f64,
            };
            let exit_side =
                get_exit_side_from_rect(&exit_point, current_target.get_bounding_client_rect());
            let padded_exit_points = get_padded_exit_points(&exit_point, exit_side, None);
            let hover_target_points = get_points_from_rect(hover_target.get_bounding_client_rect());
            let grace_area = get_hull(
                padded_exit_points
                    .into_iter()
                    .chain(hover_target_points)
                    .collect(),
            );
            pointer_grace_area.set(Some(grace_area));
            on_pointer_in_transit_change.emit(true);
        }
    });

    let trigger = use_state_eq(|| None);
    let content = use_state_eq(|| None);
    use_effect({
        let trigger_ref = context.trigger_ref.clone();
        let content_ref = content_ref.clone();
        let trigger = trigger.clone();
        let content = content.clone();

        move || {
            trigger.set(trigger_ref.cast::<web_sys::HtmlElement>());
            content.set(content_ref.cast::<web_sys::HtmlElement>());
        }
    });

    use_effect_with((), {
        let handle_remove_grace_area = handle_remove_grace_area.clone();

        move |_| {
            move || {
                handle_remove_grace_area.emit(());
            }
        }
    });

    use_effect_with(
        (trigger.clone(), content.clone(), handle_create_grace_area),
        move |(trigger, content, handle_create_grace_area)| {
            let mut cleanup: Option<Box<dyn Fn()>> = None;

            if let (Some(trigger), Some(content)) = ((**trigger).clone(), (**content).clone()) {
                let handle_trigger_leave: Closure<dyn Fn(PointerEvent)> = Closure::new({
                    let trigger = trigger.clone();
                    let content = content.clone();
                    let handle_create_grace_area = handle_create_grace_area.clone();

                    move |event| {
                        handle_create_grace_area.emit((event, content.clone(), trigger.clone()))
                    }
                });
                let handle_content_leave: Closure<dyn Fn(PointerEvent)> = Closure::new({
                    let trigger = trigger.clone();
                    let content = content.clone();
                    let handle_create_grace_area = handle_create_grace_area.clone();

                    move |event| {
                        handle_create_grace_area.emit((event, trigger.clone(), content.clone()))
                    }
                });

                trigger
                    .add_event_listener_with_callback(
                        "pointerleave",
                        handle_trigger_leave.as_ref().unchecked_ref(),
                    )
                    .expect("Pointer leave event listener should be added.");
                content
                    .add_event_listener_with_callback(
                        "pointerleave",
                        handle_content_leave.as_ref().unchecked_ref(),
                    )
                    .expect("Pointer leave event listener should be added.");

                cleanup = Some(Box::new(move || {
                    trigger
                        .remove_event_listener_with_callback(
                            "pointerleave",
                            handle_trigger_leave.as_ref().unchecked_ref(),
                        )
                        .expect("Pointer leave event listener should be removed.");
                    content
                        .remove_event_listener_with_callback(
                            "pointerleave",
                            handle_content_leave.as_ref().unchecked_ref(),
                        )
                        .expect("Pointer leave event listener should be removed.");
                }));
            }

            move || {
                if let Some(cleanup) = cleanup {
                    cleanup();
                }
            }
        },
    );

    use_effect_with(
        (
            trigger.clone(),
            content.clone(),
            pointer_grace_area,
            handle_remove_grace_area.clone(),
            context.on_close,
        ),
        |(trigger, content, pointer_grace_area, handle_remove_grace_area, on_close)| {
            let mut cleanup: Option<Box<dyn Fn()>> = None;

            if let Some(pointer_grace_area) = pointer_grace_area.as_ref() {
                let document = window()
                    .expect("Window should exist.")
                    .document()
                    .expect("Document should exist.");

                let handle_track_pointer_grace: Closure<dyn Fn(PointerEvent)> = Closure::new({
                    let trigger = (*trigger).clone();
                    let content = (*content).clone();
                    let pointer_grace_area = pointer_grace_area.clone();
                    let handle_remove_grace_area = handle_remove_grace_area.clone();
                    let on_close = on_close.clone();

                    move |event: PointerEvent| {
                        let target = event.target_unchecked_into::<web_sys::HtmlElement>();
                        let pointer_position = Point {
                            x: event.client_x() as f64,
                            y: event.client_y() as f64,
                        };
                        let has_entered_target = trigger
                            .as_ref()
                            .is_some_and(|trigger| trigger.contains(Some(&target)))
                            || content
                                .as_ref()
                                .is_some_and(|content| content.contains(Some(&target)));
                        let is_pointer_outside_grace_area =
                            !is_point_in_polygon(pointer_position, &pointer_grace_area);

                        if has_entered_target {
                            handle_remove_grace_area.emit(());
                        } else if is_pointer_outside_grace_area {
                            handle_remove_grace_area.emit(());
                            on_close.emit(());
                        }
                    }
                });

                document
                    .add_event_listener_with_callback(
                        "pointermove",
                        handle_track_pointer_grace.as_ref().unchecked_ref(),
                    )
                    .expect("Pointer move event listener should be added.");

                cleanup = Some(Box::new(move || {
                    document
                        .remove_event_listener_with_callback(
                            "pointermove",
                            handle_track_pointer_grace.as_ref().unchecked_ref(),
                        )
                        .expect("Pointer move event listener should be removed.");
                }));
            }

            move || {
                if let Some(cleanup) = cleanup {
                    cleanup();
                }
            }
        },
    );

    html! {
        <TooltipContentImpl
            side={props.side}
            side_offset={props.side_offset}
            align={props.align}
            align_offset={props.align_offset}
            arrow_padding={props.arrow_padding}
            avoid_collisions={props.avoid_collisions}
            collision_boundary={props.collision_boundary.clone()}
            collision_padding={props.collision_padding.clone()}
            sticky={props.sticky}
            hide_when_detached={props.hide_when_detached}
            update_position_strategy={props.update_position_strategy}

            dir={props.dir.clone()}
            class={props.class.clone()}
            id={props.id.clone()}
            role={props.role.clone()}
            style={props.style.clone()}

            node_ref={composed_ref}
            attributes={props.attributes.clone()}
            as_child={props.as_child.clone()}
        >
            {props.children.clone()}
        </TooltipContentImpl>
    }
}

#[derive(Clone, Default, PartialEq)]
struct VisuallyHiddenContentContextValue {
    is_inside: bool,
}

#[derive(PartialEq, Properties)]
struct TooltipContentImplProps {
    // Props from `DismissableLayer`
    // TODO: on_escape_key_down, on_pointer_down_outside

    // Props from `PopperContent`
    #[prop_or(Side::Bottom)]
    pub side: Side,
    #[prop_or(0.0)]
    pub side_offset: f64,
    #[prop_or(Align::Center)]
    pub align: Align,
    #[prop_or(0.0)]
    pub align_offset: f64,
    #[prop_or(0.0)]
    pub arrow_padding: f64,
    #[prop_or(true)]
    pub avoid_collisions: bool,
    #[prop_or_default]
    pub collision_boundary: Vec<web_sys::Element>,
    #[prop_or(Padding::All(0.0))]
    pub collision_padding: Padding,
    #[prop_or(Sticky::Partial)]
    pub sticky: Sticky,
    #[prop_or(false)]
    pub hide_when_detached: bool,
    #[prop_or(UpdatePositionStrategy::Optimized)]
    pub update_position_strategy: UpdatePositionStrategy,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub dir: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub role: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<TooltipContentChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

type TooltipContentImplChildProps = PopperContentChildProps;

#[function_component]
fn TooltipContentImpl(props: &TooltipContentImplProps) -> Html {
    let context = use_context::<TooltipContextValue>().expect("Tooltip context required.");

    // Close this tooltip if another one opens.
    use_effect_with(context.on_close.clone(), |on_close| {
        let document = window()
            .expect("Window should exist.")
            .document()
            .expect("Document should exist.");

        let handle_tooltip_open: Closure<dyn Fn()> = Closure::new({
            let on_close = on_close.clone();

            move || {
                on_close.emit(());
            }
        });

        document
            .add_event_listener_with_callback(
                TOOLTIP_OPEN,
                handle_tooltip_open.as_ref().unchecked_ref(),
            )
            .expect("Tooltip open event handler should be added.");

        move || {
            document
                .remove_event_listener_with_callback(
                    TOOLTIP_OPEN,
                    handle_tooltip_open.as_ref().unchecked_ref(),
                )
                .expect("Tooltip open event handler should be removed.");
        }
    });

    // Close the tooltip if the trigger is scrolled.
    use_effect_with(
        (context.trigger_ref, context.on_close),
        |(trigger_ref, on_close)| {
            let mut cleanup: Option<Box<dyn Fn()>> = None;

            if let Some(trigger) = trigger_ref.get() {
                let window = window().expect("Window should exist.");

                let handle_scroll: Closure<dyn Fn(Event)> = Closure::new({
                    let on_close = on_close.clone();

                    move |event: Event| {
                        let target = event.target_dyn_into::<web_sys::HtmlElement>();
                        if let Some(target) = target {
                            if target.contains(Some(&trigger)) {
                                on_close.emit(());
                            }
                        }
                    }
                });

                window
                    .add_event_listener_with_callback(
                        "scroll",
                        handle_scroll.as_ref().unchecked_ref(),
                    )
                    .expect("Scroll event handler should be added.");

                cleanup = Some(Box::new(move || {
                    window
                        .remove_event_listener_with_callback(
                            "scroll",
                            handle_scroll.as_ref().unchecked_ref(),
                        )
                        .expect("Scroll event handler should be removed.");
                }));
            }

            move || {
                if let Some(cleanup) = cleanup {
                    cleanup();
                }
            }
        },
    );

    let context_value = use_memo((), |_| VisuallyHiddenContentContextValue {
        is_inside: true,
    });

    let aria_label = props
        .attributes
        .as_ref()
        .and_then(|attributes| attributes.get("aria-label").cloned())
        .flatten();

    html! {
        // TODO: DismissableLayer
        <PopperContent
            side={props.side}
            side_offset={props.side_offset}
            align={props.align}
            align_offset={props.align_offset}
            arrow_padding={props.arrow_padding}
            avoid_collisions={props.avoid_collisions}
            collision_boundary={props.collision_boundary.clone()}
            collision_padding={props.collision_padding.clone()}
            sticky={props.sticky}
            hide_when_detached={props.hide_when_detached}
            update_position_strategy={props.update_position_strategy}

            dir={props.dir.clone()}
            class={props.class.clone()}
            id={props.id.clone()}
            role={props.role.clone()}
            style={props.style.clone().with_defaults([
                // Re-namespace exposed content custom properties.
                ("--radix-tooltip-content-transform-origin", "var(--radix-popper-transform-origin)"),
                ("--radix-tooltip-content-available-width", "var(--radix-popper-available-width)"),
                ("--radix-tooltip-content-available-height", "var(--radix-popper-available-height)"),
                ("--radix-tooltip-trigger-width", "var(--radix-popper-anchor-width)"),
                ("--radix-tooltip-trigger-height", "var(--radix-popper-anchor-height)"),
            ])}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone().with_defaults([
                ("data-state", context.state_attribute.to_string())
            ])}
            as_child={props.as_child.clone()}
        >
            {props.children.clone()}
            <ContextProvider<VisuallyHiddenContentContextValue> context={(*context_value).clone()}>
                <VisuallyHidden id={context.content_id} attributes={[("role", "tooltip")]}>
                    {aria_label.map(Html::from).unwrap_or_else(|| props.children.clone())}
                </VisuallyHidden>
            </ContextProvider<VisuallyHiddenContentContextValue>>
        </PopperContent>
    }
}

#[derive(PartialEq, Properties)]
pub struct TooltipArrowProps {
    // Props from `PopperArrow`
    #[prop_or(10.0)]
    pub width: f64,
    #[prop_or(5.0)]
    pub height: f64,

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
    pub as_child: Option<Callback<TooltipArrowChildProps, Html>>,
}

pub type TooltipArrowChildProps = PopperArrowChildProps;

#[function_component]
pub fn TooltipArrow(props: &TooltipArrowProps) -> Html {
    let visually_hidden_content_context =
        use_context::<VisuallyHiddenContentContextValue>().unwrap_or_default();

    // If the arrow is inside the `VisuallyHidden`, we don't want to render it all to
    // prevent issues in positioning the arrow due to the duplicate.
    html! {
        if !visually_hidden_content_context.is_inside {
            <PopperArrow
                width={props.width}
                height={props.height}

                class={props.class.clone()}
                id={props.id.clone()}
                style={props.style.clone()}

                node_ref={props.node_ref.clone()}
                attributes={props.attributes.clone()}
                as_child={props.as_child.clone()}
            />
        }
    }
}

fn get_exit_side_from_rect(point: &Point, rect: web_sys::DomRect) -> Side {
    let top = (rect.top() - point.y).abs();
    let bottom = (rect.bottom() - point.y).abs();
    let right = (rect.right() - point.x).abs();
    let left = (rect.left() - point.x).abs();

    let min = top.min(bottom).min(right).min(left);

    if min == left {
        Side::Left
    } else if min == right {
        Side::Right
    } else if min == top {
        Side::Top
    } else if min == bottom {
        Side::Bottom
    } else {
        unreachable!()
    }
}

fn get_padded_exit_points(exit_point: &Point, exit_side: Side, padding: Option<f64>) -> Vec<Point> {
    let padding = padding.unwrap_or(5.0);

    match exit_side {
        Side::Top => {
            vec![
                Point {
                    x: exit_point.x - padding,
                    y: exit_point.y + padding,
                },
                Point {
                    x: exit_point.x + padding,
                    y: exit_point.y + padding,
                },
            ]
        }
        Side::Right => {
            vec![
                Point {
                    x: exit_point.x - padding,
                    y: exit_point.y - padding,
                },
                Point {
                    x: exit_point.x - padding,
                    y: exit_point.y + padding,
                },
            ]
        }
        Side::Bottom => {
            vec![
                Point {
                    x: exit_point.x - padding,
                    y: exit_point.y - padding,
                },
                Point {
                    x: exit_point.x + padding,
                    y: exit_point.y - padding,
                },
            ]
        }
        Side::Left => {
            vec![
                Point {
                    x: exit_point.x + padding,
                    y: exit_point.y - padding,
                },
                Point {
                    x: exit_point.x + padding,
                    y: exit_point.y + padding,
                },
            ]
        }
    }
}

fn get_points_from_rect(rect: web_sys::DomRect) -> Vec<Point> {
    vec![
        Point {
            x: rect.left(),
            y: rect.top(),
        },
        Point {
            x: rect.right(),
            y: rect.top(),
        },
        Point {
            x: rect.right(),
            y: rect.bottom(),
        },
        Point {
            x: rect.left(),
            y: rect.bottom(),
        },
    ]
}

/// Determine if a point is inside of a polygon.
/// Based on https://github.com/substack/point-in-polygon
fn is_point_in_polygon(point: Point, polygon: &Polygon) -> bool {
    let x = point.x;
    let y = point.y;

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

/// Returns a new vector of points representing the convex hull of the given set of points.
/// https://www.nayuki.io/page/convex-hull-algorithm
fn get_hull(points: Vec<Point>) -> Vec<Point> {
    let mut new_points = points.clone();

    new_points.sort_by(|a, b| {
        if a.x < b.x {
            Ordering::Less
        } else if a.x > b.x {
            Ordering::Greater
        } else if a.y < b.y {
            Ordering::Less
        } else if a.y > b.y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    get_hull_presorted(new_points)
}

/// Returns the convex hull, assuming that each points[i] <= points[i + 1]. Runs in O(n) time.
fn get_hull_presorted(points: Vec<Point>) -> Vec<Point> {
    if points.len() <= 1 {
        return points;
    }

    let mut upper_hull: Vec<Point> = vec![];
    for p in points.iter() {
        while upper_hull.len() >= 2 {
            let q = &upper_hull[upper_hull.len() - 1];
            let r = &upper_hull[upper_hull.len() - 2];

            if (q.x - r.x) * (p.y - r.y) >= (q.y - r.y) * (p.x - r.x) {
                upper_hull.pop();
            } else {
                break;
            }
        }
        upper_hull.push(p.clone());
    }
    upper_hull.pop();

    let mut lower_hull: Vec<Point> = vec![];
    for p in points.iter().rev() {
        while lower_hull.len() >= 2 {
            let q = &lower_hull[lower_hull.len() - 1];
            let r = &lower_hull[lower_hull.len() - 2];

            if (q.x - r.x) * (p.y - r.y) >= (q.y - r.y) * (p.x - r.x) {
                lower_hull.pop();
            } else {
                break;
            }
        }
        lower_hull.push(p.clone());
    }
    lower_hull.pop();

    if upper_hull.len() == 1
        && lower_hull.len() == 1
        && upper_hull[0].x == lower_hull[0].x
        && upper_hull[0].y == lower_hull[0].y
    {
        upper_hull
    } else {
        upper_hull.into_iter().chain(lower_hull).collect()
    }
}
