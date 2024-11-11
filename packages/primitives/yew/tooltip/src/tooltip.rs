use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

use radix_yew_id::use_id;
use radix_yew_popper::{
    Align, Padding, Popper, PopperAnchor, PopperAnchorChildProps, PopperArrow,
    PopperArrowChildProps, PopperContent, PopperContentChildProps, Side, Sticky,
    UpdatePositionStrategy,
};
use radix_yew_primitive::compose_callbacks;
use radix_yew_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use radix_yew_visually_hidden::VisuallyHidden;
use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    window, CustomEvent,
};
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};

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
                window()
                    .expect("Window should exist.")
                    .clear_timeout_with_handle(open_timer);
                *open_timer_ref.borrow_mut() = 0;

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
    pub style: Option<String>,

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
    pub data_state: String,
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Option<String>,

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
    ///  Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop_or_default]
    pub force_mount: Option<bool>,

    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TooltipPortal(props: &TooltipPortalProps) -> Html {
    let context_value = use_memo(props.force_mount, |force_mount| TooltipPortalContextValue {
        force_mount: *force_mount,
    });

    html! {
        <ContextProvider<TooltipPortalContextValue> context={(*context_value).clone()}>
            // TODO: Presence, Portal
            {props.children.clone()}
        </ContextProvider<TooltipPortalContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct TooltipContentProps {
    ///  Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
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
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<PopperContentChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TooltipContent(props: &TooltipContentProps) -> Html {
    let portal_context =
        use_context::<TooltipPortalContextValue>().expect("Tooltip portal context required.");
    let context = use_context::<TooltipContextValue>().expect("Tooltip context required.");

    let _force_mount = props.force_mount.or(portal_context.force_mount);

    html! {
        // TODO: Presence
        if context.disable_hoverable_content {
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

                node_ref={props.node_ref.clone()}
                attributes={props.attributes.clone()}
                as_child={props.as_child.clone()}
            />
        } else {
            <TooltipContentHoverable
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

                node_ref={props.node_ref.clone()}
                attributes={props.attributes.clone()}
                as_child={props.as_child.clone()}
            />
        }
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
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<PopperContentChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn TooltipContentHoverable(props: &TooltipContentHoverableProps) -> Html {
    let _context = use_context::<TooltipContextValue>().expect("Tooltip context required.");
    let _provider_context =
        use_context::<TooltipProviderContextValue>().expect("Tooltip provider context required.");
    let content_ref = use_node_ref();
    let composed_ref = use_composed_ref(&[props.node_ref.clone(), content_ref.clone()]);
    let _pointer_grace_area = use_state_eq(|| None::<Polygon>);

    // TODO: effects

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
        />
    }
}

#[derive(Clone, PartialEq)]
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
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<PopperContentChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

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
            style={format!(
                // Re-namespace exposed content custom properties.
                "\
                --radix-tooltip-content-transform-origin: var(--radix-popper-transform-origin);\
                --radix-tooltip-content-available-width: var(--radix-popper-available-width);\
                --radix-tooltip-content-available-height: var(--radix-popper-available-height);\
                --radix-tooltip-trigger-width: var(--radix-popper-anchor-width);\
                --radix-tooltip-trigger-height: var(--radix-popper-anchor-height);\
                {}",
                props.style.clone().unwrap_or_default()
            )}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone().with_defaults([
                ("data-state", context.state_attribute.to_string())
            ])}
            as_child={props.as_child.clone()}
        >
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
    pub style: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<PopperArrowChildProps, Html>>,
}

#[function_component]
pub fn TooltipArrow(props: &TooltipArrowProps) -> Html {
    let visually_hidden_content_context = use_context::<VisuallyHiddenContentContextValue>()
        .expect("Visually hidden content context required.");

    html! {
        if visually_hidden_content_context.is_inside {
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
