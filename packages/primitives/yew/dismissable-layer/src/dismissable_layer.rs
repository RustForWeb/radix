use std::{cell::RefCell, rc::Rc};

use web_sys::CustomEvent;
use yew::prelude::*;

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
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<DismissableLayerChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct DismissableLayerChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: String,
}

impl DismissableLayerChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <div
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
            >
                {children}
            </div>
        }
    }
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
                node_ref={props.node_ref.clone()}
                id={props.id.clone()}
                class={props.class.clone()}
                style={props.style.clone()}
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
    // TODO: owner_document, force?
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), node_ref.clone()]);
    // TODO
    let is_body_pointer_events_disabled = !context
        .layers_with_outside_pointer_events_disabled
        .borrow()
        .is_empty();
    // let is_pointer_events_enabled =

    let child_props = DismissableLayerChildProps {
        node_ref: composed_refs,
        id: props.id.clone(),
        class: props.class.clone(),
        style: format!(
            "{}{}",
            is_body_pointer_events_disabled
                .then_some("".to_string())
                .unwrap_or_default(),
            props.style.clone().unwrap_or_default()
        ),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(PartialEq, Properties)]
pub struct DismissableLayerBranchProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<DismissableLayerBranchChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct DismissableLayerBranchChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
}

impl DismissableLayerBranchChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <div
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
            >
                {children}
            </div>
        }
    }
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
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

pub struct PointerDownOutsideEventDetail {
    pub original_event: PointerEvent,
}

pub type PointerDownOutsideEvent = CustomEvent;

pub struct FocusOutsideEventDetail {
    pub original_event: FocusEvent,
}

pub type FocusOutsideEvent = CustomEvent;

pub enum InteractOutsideEvent {
    PointerDownOutside(PointerDownOutsideEvent),
    FocusOutside(FocusOutsideEvent),
}
