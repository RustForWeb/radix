use std::{cell::RefCell, rc::Rc};

use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast, JsValue},
    window, AddEventListenerOptions, CustomEvent, CustomEventInit,
};
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};
use yew_style::Style;

#[expect(unused)]
const CONTEXT_UPDATE: &str = "dismissableLayer.update";
const _POINTER_DOWN_OUTSIDE: &str = "dismissableLayer.pointerDownOutside";
const FOCUS_OUTSIDE: &str = "dismissableLayer.focusOutside";

#[derive(Clone, PartialEq)]
struct DismissableLayerContextValue {
    layers: Rc<RefCell<Vec<web_sys::Element>>>,
    layers_with_outside_pointer_events_disabled: Rc<RefCell<Vec<web_sys::Element>>>,
    branches: Rc<RefCell<Vec<web_sys::Element>>>,
}

#[derive(PartialEq, Properties)]
pub struct DismissableLayerProps {
    ///  When `true`, hover/focus/click interactions will be disabled on elements outside
    /// the `DismissableLayer`. Users will need to click twice on outside elements to
    /// interact with them: once to close the `DismissableLayer`, and again to trigger the element.
    #[prop_or(false)]
    pub disable_outside_pointer_events: bool,
    /// Event handler called when the escape key is down.
    /// Can be prevented.
    #[prop_or_default]
    pub on_escape_key_down: Callback<KeyboardEvent>,
    /// Event handler called when the a `pointerdown` event happens outside of the `DismissableLayer`.
    /// Can be prevented.
    #[prop_or_default]
    pub on_pointer_down_outside: Callback<PointerDownOutsideEvent>,
    /// Event handler called when the focus moves outside of the `DismissableLayer`.
    /// Can be prevented.
    #[prop_or_default]
    pub on_focus_outside: Callback<FocusOutsideEvent>,
    /// Event handler called when an interaction happens outside the `DismissableLayer`.
    /// Specifically, when a `pointerdown` event happens outside or focus moves outside of it.
    /// Can be prevented.
    #[prop_or_default]
    pub on_interact_outside: Callback<InteractOutsideEvent>,
    /// Handler called when the `DismissableLayer` should be dismissed.
    #[prop_or_default]
    pub on_dismiss: Callback<()>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Event handler attributes
    #[prop_or_default]
    pub on_key_down: Callback<KeyboardEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<DismissableLayerChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct DismissableLayerChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Style,

    // Event handler attributes
    pub onkeydown: Callback<KeyboardEvent>,
}

#[function_component]
pub fn DismissableLayer(props: &DismissableLayerProps) -> Html {
    let layers = use_mut_ref(Vec::new);
    let layers_with_outside_pointer_events_disabled = use_mut_ref(Vec::new);
    let branches = use_mut_ref(Vec::new);

    let context_value = use_memo((), |_| DismissableLayerContextValue {
        layers,
        layers_with_outside_pointer_events_disabled,
        branches,
    });

    html! {
        // Unlike React, Yew's `use_context` does not provide a default value without a `ContextProvider`.
        <ContextProvider<DismissableLayerContextValue> context={(*context_value).clone()}>
            <DismissableLayerImpl
                disable_outside_pointer_events={props.disable_outside_pointer_events}
                on_escape_key_down={props.on_escape_key_down.clone()}
                on_pointer_down_outside={props.on_pointer_down_outside.clone()}
                on_focus_outside={props.on_focus_outside.clone()}
                on_interact_outside={props.on_interact_outside.clone()}
                on_dismiss={props.on_dismiss.clone()}

                class={props.class.clone()}
                id={props.id.clone()}
                style={props.style.clone()}

                node_ref={props.node_ref.clone()}
                attributes={props.attributes.clone()}
                as_child={props.as_child.clone()}
            >
                {props.children.clone()}
            </DismissableLayerImpl>
        </ContextProvider<DismissableLayerContextValue>>
    }
}

#[function_component]
pub fn DismissableLayerImpl(props: &DismissableLayerProps) -> Html {
    let context = use_context::<DismissableLayerContextValue>()
        .expect("Dismissable layer context is required.");

    let node_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), node_ref.clone()]);
    let node = use_state_eq(|| None);
    use_effect_with(node_ref.clone(), {
        let node = node.clone();

        move |node_ref| {
            node.set(node_ref.cast::<web_sys::Element>());
        }
    });

    let owned_document = use_memo(node.clone(), |node| {
        node.as_ref()
            .and_then(|node| node.owner_document())
            .unwrap_or(
                window()
                    .expect("Window should exist.")
                    .document()
                    .expect("Document should exist"),
            )
    });
    let _force = use_state(|| 0);

    let (is_body_pointer_events_disabled, is_pointer_events_enabled) = {
        // Drop borrowed layers immediately after these calculations.
        let layers = context.layers.borrow();
        let layers_with_outside_pointer_events_disabled =
            context.layers_with_outside_pointer_events_disabled.borrow();

        let highest_layer_with_outside_pointer_events_disabled =
            layers_with_outside_pointer_events_disabled.last();
        let highest_layer_with_outside_pointer_events_disabled =
            highest_layer_with_outside_pointer_events_disabled.and_then(
                |highest_layer_with_oytside_pointer_events_disabled| {
                    layers.iter().position(|layer| {
                        layer == highest_layer_with_oytside_pointer_events_disabled
                    })
                },
            );
        let index = node
            .as_ref()
            .and_then(|node| layers.iter().position(|layer| layer == node));
        let is_body_pointer_events_disabled = !context
            .layers_with_outside_pointer_events_disabled
            .borrow()
            .is_empty();
        let is_pointer_events_enabled =
            match (index, highest_layer_with_outside_pointer_events_disabled) {
                (Some(index), Some(highest_layer_with_outside_pointer_events_disabled)) => {
                    index >= highest_layer_with_outside_pointer_events_disabled
                }
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (None, None) => true,
            };

        (is_body_pointer_events_disabled, is_pointer_events_enabled)
    };

    let handle_pointer_down_outside = use_callback((), |_event, ()| {
        // TODO
    });
    let _pointer_down_outside =
        use_pointer_down_outside(handle_pointer_down_outside, owned_document.clone());

    let handle_focus_outside = use_callback((), |_event, ()| {
        // TODO
    });
    let _focus_outside = use_focus_outside(handle_focus_outside, owned_document);

    // TODO: use_escape_keydown

    // TODO: effects

    let child_props = DismissableLayerChildProps {
        node_ref: composed_refs,
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        id: props.id.clone(),
        style: props.style.clone().with_defaults([(
            "pointer-events",
            is_body_pointer_events_disabled.then_some(if is_pointer_events_enabled {
                "auto"
            } else {
                "none"
            }),
        )]),

        // Event handler attributes
        onkeydown: props.on_key_down.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(PartialEq, Properties)]
pub struct DismissableLayerBranchProps {
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
    pub as_child: Option<Callback<DismissableLayerBranchChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct DismissableLayerBranchChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn DismissableLayerBranch(props: &DismissableLayerBranchProps) -> Html {
    let context = use_context::<DismissableLayerContextValue>()
        .expect("Dismissable layer context is required.");
    let node_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), node_ref.clone()]);

    use_effect_with(node_ref, |node_ref| {
        let mut cleanup: Option<Box<dyn Fn()>> = None;

        if let Some(node) = node_ref.cast::<web_sys::Element>() {
            context.branches.borrow_mut().push(node.clone());

            cleanup = Some(Box::new(move || {
                context
                    .branches
                    .borrow_mut()
                    .retain(|branch| *branch != node);
            }));
        }

        move || {
            if let Some(cleanup) = cleanup {
                cleanup();
            }
        }
    });

    let child_props = DismissableLayerBranchChildProps {
        node_ref: composed_refs,
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        id: props.id.clone(),
        style: props.style.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

trait EventDetail<E>: Into<JsValue> {
    fn original_event(&self) -> E;
}

pub struct PointerDownOutsideEventDetail {
    pub original_event: PointerEvent,
}

impl EventDetail<PointerEvent> for PointerDownOutsideEventDetail {
    fn original_event(&self) -> PointerEvent {
        self.original_event.clone()
    }
}

impl From<PointerDownOutsideEventDetail> for JsValue {
    fn from(_value: PointerDownOutsideEventDetail) -> Self {
        todo!("PointerDownOutsideEventDetail to Jsvalue")
    }
}

pub type PointerDownOutsideEvent = CustomEvent;

pub struct FocusOutsideEventDetail {
    pub original_event: FocusEvent,
}

pub type FocusOutsideEvent = CustomEvent;

impl EventDetail<FocusEvent> for FocusOutsideEventDetail {
    fn original_event(&self) -> FocusEvent {
        self.original_event.clone()
    }
}

impl From<FocusOutsideEventDetail> for JsValue {
    fn from(_value: FocusOutsideEventDetail) -> Self {
        todo!("FocusOutsideEventDetail to JsValue")
    }
}

pub enum InteractOutsideEvent {
    PointerDownOutside(PointerDownOutsideEvent),
    FocusOutside(FocusOutsideEvent),
}

struct UsePointerDownOutsideReturn {
    // on_pointer_down_capture: Callback<()>,
}

/// Listens for `pointerdown` outside a subtree. We use `pointerdown` rather than `pointerup` to mimic layer dismissing behaviour present in OS.
/// Returns props to pass to the node we want to check for outside events.
#[hook]
fn use_pointer_down_outside(
    _on_pointer_down_outside: Callback<PointerDownOutsideEvent>,
    _owner_document: Rc<web_sys::Document>,
) -> UsePointerDownOutsideReturn {
    UsePointerDownOutsideReturn {
        // on_pointer_down_capture:
    }
}

struct UseFocusOutsideReturn {
    // on_focus_capture: Callback<()>,
    // on_blur_capture: Callback<()>,
}

/// Listens for when focus happens outside a subtree.
/// Returns props to pass to the root (node) of the subtree we want to check.
#[hook]
fn use_focus_outside(
    on_focus_outside: Callback<FocusOutsideEvent>,
    owner_document: Rc<web_sys::Document>,
) -> UseFocusOutsideReturn {
    let handle_focus_outside = on_focus_outside;
    let is_focus_inside_tree_ref = use_mut_ref(|| false);

    use_effect_with(
        (owner_document, handle_focus_outside),
        |(owner_document, handle_focus_outside)| {
            let owner_document = (*owner_document).clone();
            let handle_focus_outside = handle_focus_outside.clone();

            let handle_focus: Rc<Closure<dyn Fn(FocusEvent)>> =
                Rc::new(Closure::new(move |event: FocusEvent| {
                    if event.target().is_some() && !*is_focus_inside_tree_ref.borrow() {
                        let event_detail = FocusOutsideEventDetail {
                            original_event: event,
                        };

                        handle_and_dispatch_custom_event(
                            FOCUS_OUTSIDE,
                            Some(handle_focus_outside.clone()),
                            event_detail,
                        );
                    }
                }));

            owner_document
                .add_event_listener_with_callback(
                    "focusin",
                    (*handle_focus).as_ref().unchecked_ref(),
                )
                .expect("Focus in event listener should be added.");

            move || {
                owner_document
                    .remove_event_listener_with_callback(
                        "focusin",
                        (*handle_focus).as_ref().unchecked_ref(),
                    )
                    .expect("Focus in event listener should be removed.");
            }
        },
    );

    UseFocusOutsideReturn {
        // on_focus_capture:
        // on_blur_capture:
    }
}

fn _dispatch_update() {
    let event =
        CustomEvent::new(CONTEXT_UPDATE).expect("Context update event should be instantiated.");

    window()
        .expect("Window should exist")
        .document()
        .expect("Document should exist.")
        .dispatch_event(&event)
        .expect("Context update event should be dispatched.");
}

fn handle_and_dispatch_custom_event<O, D>(
    name: &str,
    handler: Option<Callback<CustomEvent>>,
    detail: D,
) where
    O: Into<Event>,
    D: EventDetail<O>,
{
    let original_event: Event = detail.original_event().into();
    let target = original_event.target().expect("Event should have target.");

    let event_init = CustomEventInit::new();
    event_init.set_bubbles(false);
    event_init.set_cancelable(true);
    event_init.set_detail(&detail.into());
    let event = CustomEvent::new_with_event_init_dict(name, &event_init)
        .expect("Event should be instantiated.");

    if let Some(handler) = handler {
        let closure = Closure::once_into_js(move |event| {
            handler.emit(event);
        });

        let options = AddEventListenerOptions::new();
        options.set_once(true);
        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                name,
                closure.unchecked_ref(),
                &options,
            )
            .expect("Event listener should be added.");
    }

    target
        .dispatch_event(&event)
        .expect("Event should be dispatched.");
}
