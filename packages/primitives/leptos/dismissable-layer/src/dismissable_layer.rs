use std::{
    sync::{Arc, LazyLock, Mutex, atomic::AtomicBool},
    time::Duration,
};

use crate::object_key::{ObjectId, ObjectKey};
use indexmap::{IndexMap, IndexSet};
use leptos::{
    ev::{CustomEvent, Event, FocusEvent, KeyboardEvent, PointerEvent},
    html,
    prelude::*,
    web_sys::{AddEventListenerOptions, CustomEventInit, Element, EventListenerOptions},
};
use leptos_maybe_callback::MaybeCallback;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_primitive::Primitive;
use radix_leptos_use_escape_keydown::use_escape_keydown;
use send_wrapper::SendWrapper;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, prelude::Closure};
use wasm_bindgen::{JsValue, intern};

mod strings {
    use wasm_bindgen::intern;

    const POINTER_DOWN_OUTSIDE: &str = "dismissableLayer.pointerDownOutside";
    const FOCUS_OUTSIDE: &str = "dismissableLayer.focusOutside";

    pub fn pointer_down_outside() -> &'static str {
        intern(POINTER_DOWN_OUTSIDE)
    }

    pub fn focus_outside() -> &'static str {
        intern(FOCUS_OUTSIDE)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PointerDownOutsideEventDetail {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub original_event: PointerEvent,
}

pub type PointerDownOutsideEvent = CustomEvent;

#[derive(Serialize, Deserialize)]
pub struct FocusOutsideEventDetail {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub original_event: FocusEvent,
}

pub type FocusOutsideEvent = CustomEvent;

pub type InteractOutsideEvent = CustomEvent;

pub trait DismissableLayerEventDetail: Serialize {
    fn original_event(&self) -> &Event;
}

impl DismissableLayerEventDetail for PointerDownOutsideEventDetail {
    fn original_event(&self) -> &Event {
        &self.original_event
    }
}

impl DismissableLayerEventDetail for FocusOutsideEventDetail {
    fn original_event(&self) -> &Event {
        &self.original_event
    }
}

#[derive(Debug)]
struct LayerInfo {
    outside_pointer_events_disabled: bool,
}

#[derive(Debug, Default)]
struct DismissableLayerContextInner {
    layers: IndexMap<ObjectKey<Element>, LayerInfo>,
    branches: IndexSet<ObjectKey<Element>>,
}

type DismissableLayerContext = RwSignal<DismissableLayerContextInner>;
// #[derive(Debug, Default, Clone)]
// struct DismissableLayerContext {
//     inner: RwSignal<DismissableLayerContextInner>,
// }

fn use_or_provide_context<T: Default + Clone + Send + Sync + 'static>() -> T {
    if let Some(context) = with_context(|ctx: &T| ctx.clone()) {
        context
    } else {
        let context = T::default();
        provide_context(context.clone());
        context
    }
}

static ORIGINAL_BODY_POINTER_EVENTS: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new(String::new()));

#[component]
pub fn DismissableLayer(
    /// When `true`, hover/focus/click interactions will be disabled on elements outside
    /// the `DismissableLayer`. Users will need to click twice on outside elements to
    /// interact with them: once to close the `DismissableLayer`, and again to trigger the element.
    #[prop(into, optional, default = false.into())]
    disable_outside_pointer_events: MaybeProp<bool>,
    /// Event handler called when the escape key is down.
    /// Can be prevented.
    #[prop(into, optional)]
    on_escape_key_down: MaybeCallback<KeyboardEvent>,
    /// Event handler called when a `pointerdown` event happens outside of the `DismissableLayer`.
    /// Can be prevented.
    #[prop(into, optional)]
    on_pointer_down_outside: MaybeCallback<PointerDownOutsideEvent>,
    /// Event handler called when the focus moves outside of the `DismissableLayer`.
    /// Can be prevented.
    #[prop(into, optional)]
    on_focus_outside: MaybeCallback<FocusOutsideEvent>,
    /// Event handler called when an interaction happens outside of the `DismissableLayer`.
    /// Specifically, when a `pointerdown` event happens outside or focus moves outside of it.
    /// Can be prevented.
    #[prop(into, optional)]
    on_interact_outside: MaybeCallback<InteractOutsideEvent>,
    /// Handler called when the `DismissableLayer` should be dismissed.
    #[prop(into, optional)]
    on_dismiss: MaybeCallback<()>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_or_provide_context::<DismissableLayerContext>();
    let node = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, node]);
    let index = Signal::derive({
        let node = node.clone();

        move || {
            let Some(node) = node.get() else {
                return None;
            };

            context.with(|context| context.layers.keys().position(|key| key.as_ref() == &node))
        }
    });
    let highest_layer_with_outside_pointer_events_disabled_index = Signal::derive({
        let context = context.clone();

        move || {
            context.with(|context| {
                context
                    .layers
                    .values()
                    .rposition(|layer| layer.outside_pointer_events_disabled)
            })
        }
    });
    let is_body_pointer_events_disabled = Signal::derive({
        let context = context.clone();

        move || {
            context.with(|context| {
                context
                    .layers
                    .values()
                    .any(|layer| layer.outside_pointer_events_disabled)
            })
        }
    });

    let is_pointer_events_enabled = Signal::derive(move || {
        index.get() >= highest_layer_with_outside_pointer_events_disabled_index.get()
    });

    use_pointer_down_outside(
        MaybeCallback::from({
            let context = context.clone();
            let on_interact_outside = on_interact_outside.clone();
            let on_dismiss = on_dismiss.clone();

            move |event: PointerDownOutsideEvent| {
                let target = event
                    .target()
                    .map(|target| target.unchecked_into::<leptos::web_sys::Node>());
                let is_pointer_down_on_branch = context.with(|context| {
                    context
                        .branches
                        .iter()
                        .any(|branch| branch.contains(target.as_ref()))
                });

                if !is_pointer_events_enabled.get() || is_pointer_down_on_branch {
                    return;
                }

                on_pointer_down_outside.run(event.clone());
                on_interact_outside.run(event.clone());

                if !event.default_prevented() {
                    on_dismiss.run(());
                }
            }
        }),
        node.clone(),
    );

    use_focus_outside(
        MaybeCallback::from({
            let context = context.clone();
            let on_dismiss = on_dismiss.clone();

            move |event: FocusOutsideEvent| {
                let target = event
                    .target()
                    .map(|target| target.unchecked_into::<leptos::web_sys::Node>());

                let is_focus_in_branch = context.with(|context| {
                    context
                        .branches
                        .iter()
                        .any(|branch| branch.contains(target.as_ref()))
                });

                if is_focus_in_branch {
                    return;
                }

                on_focus_outside.run(event.clone());
                on_interact_outside.run(event.clone());

                if !event.default_prevented() {
                    on_dismiss.run(());
                }
            }
        }),
        node.clone(),
    );

    use_escape_keydown(
        MaybeCallback::from({
            let context = context.clone();

            move |event: KeyboardEvent| {
                let is_highest_layer =
                    index.get() == Some(context.with(|context| context.layers.len()) - 1);
                if !is_highest_layer {
                    return;
                }

                on_escape_key_down.run(event.clone());
                if !event.default_prevented() {
                    on_dismiss.run(());
                }
            }
        }),
        None, // TODO: This needs to accept a signal. The node value isn't yet available
    );

    Effect::watch(
        move || {
            node.track();
            disable_outside_pointer_events.track();
        },
        move |(), _, _| {
            let Some(node) = node.get() else { return };
            let disable_outside_pointer_events =
                disable_outside_pointer_events.get().unwrap_or(false);
            let owner_document = node.owner_document().unwrap();

            {
                let mut context = context.write();

                if disable_outside_pointer_events {
                    if !context
                        .layers
                        .values()
                        .any(|layer| layer.outside_pointer_events_disabled)
                    {
                        let body_styles = owner_document.body().unwrap().style();
                        *ORIGINAL_BODY_POINTER_EVENTS.lock().unwrap() = body_styles
                            .get_property_value(intern("pointer-events"))
                            .unwrap();
                        body_styles
                            .set_property(intern("pointer-events"), intern("none"))
                            .unwrap();
                    }
                }

                context.layers.insert(
                    ObjectKey::new(node.clone()).unwrap(),
                    LayerInfo {
                        outside_pointer_events_disabled: disable_outside_pointer_events,
                    },
                );
            }

            on_cleanup({
                let owner_document = SendWrapper::new(owner_document);

                move || {
                    if disable_outside_pointer_events
                        && context
                            .read()
                            .layers
                            .values()
                            .filter(|layer| layer.outside_pointer_events_disabled)
                            .skip(1)
                            .any(|_| true)
                    {
                        let body_styles = owner_document.body().unwrap().style();
                        let original_value = ORIGINAL_BODY_POINTER_EVENTS.lock().unwrap();
                        body_styles
                            .set_property(intern("pointer-events"), &original_value)
                            .unwrap();
                    }
                }
            });
        },
        false,
    );

    /*
     * We purposefully prevent combining this effect with the `disableOutsidePointerEvents` effect
     * because a change to `disableOutsidePointerEvents` would remove this layer from the stack
     * and add it to the end again so the layering order wouldn't be _creation order_.
     * We only want them to be removed from context stacks when unmounted.
     */
    Effect::watch(
        move || {
            node.track();
        },
        move |(), _, _| {
            on_cleanup(move || {
                let Some(node) = node.get() else { return };
                let key = ObjectId::for_value(&node).unwrap();
                context.write().layers.shift_remove(&key);
            });
        },
        false,
    );

    let pointer_events = Signal::derive(move || {
        match (
            is_body_pointer_events_disabled.get(),
            is_pointer_events_enabled.get(),
        ) {
            (true, true) => intern("auto"),
            (true, false) => intern("none"),
            (false, _) => "",
        }
    });

    view! {
        <Primitive
            element=html::div
            node_ref=composed_refs
            style:pointer-events=move || pointer_events.get()
        >
            {children()}
        </Primitive>
    }
}

/// Listens for `pointerdown` outside a subtree. We use `pointerdown` rather than `pointerup`
/// to mimic layer dismissing behaviour present in OS.
fn use_pointer_down_outside(
    on_pointer_down_outside: MaybeCallback<PointerDownOutsideEvent>,
    node: AnyNodeRef,
) {
    ClientOnly::new(move || {
        let handle_pointer_down_outside = Closure::<dyn Fn(PointerDownOutsideEvent)>::new(
            move |event: PointerDownOutsideEvent| {
                #[cfg(debug_assertions)]
                let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                if let Some(ref cb) = on_pointer_down_outside.0.as_ref() {
                    cb.run(event);
                }
            },
        )
        .into_js_value();

        let is_pointer_inside_react_tree = Arc::new(AtomicBool::new(false));
        let handle_click: ArcRwSignal<Option<SendWrapper<JsValue>>> = ArcRwSignal::new(None);

        Effect::watch(
            move || node.get(),
            move |node, _, _| {
                let Some(node) = node.clone() else {
                    return;
                };

                let Some(owner_document) = node.owner_document() else {
                    return;
                };

                let capture_on_pointer_down = SendWrapper::new(
                    Closure::<dyn Fn()>::new({
                        let is_pointer_inside_react_tree =
                            Arc::clone(&is_pointer_inside_react_tree);
                        move || {
                            is_pointer_inside_react_tree
                                .store(true, std::sync::atomic::Ordering::Relaxed);
                        }
                    })
                    .into_js_value(),
                );

                {
                    let options = AddEventListenerOptions::new();
                    options.set_capture(true);
                    _ = node.add_event_listener_with_callback_and_add_event_listener_options(
                        intern("pointerdown"),
                        capture_on_pointer_down.as_ref().unchecked_ref(),
                        &options,
                    );
                }

                let handle_pointer_down = SendWrapper::new(
                    Closure::<dyn Fn(PointerEvent)>::new({
                        let handle_click = handle_click.clone();
                        let owner_document = owner_document.clone();
                        let handle_pointer_down_outside = handle_pointer_down_outside.clone();
                        let is_pointer_inside_react_tree =
                            Arc::clone(&is_pointer_inside_react_tree);
                        move |event: PointerEvent| {
                            if event.target().is_some()
                                && !is_pointer_inside_react_tree
                                    .load(std::sync::atomic::Ordering::Relaxed)
                            {
                                let event_detail = PointerDownOutsideEventDetail {
                                    original_event: event.clone(),
                                };

                                let handle_and_dispatch_pointer_down_outside_event = {
                                    let handle_pointer_down_outside =
                                        handle_pointer_down_outside.clone();
                                    move || {
                                        handle_and_dispatch_custom_event(
                                            strings::pointer_down_outside(),
                                            Some(&handle_pointer_down_outside),
                                            event_detail,
                                        );
                                    }
                                };

                                /*
                                 * On touch devices, we need to wait for a click event because browsers implement
                                 * a ~350ms delay between the time the user stops touching the display and when the
                                 * browser executres events. We need to ensure we don't reactivate pointer-events within
                                 * this timeframe otherwise the browser may execute events that should have been prevented.
                                 *
                                 * Additionally, this also lets us deal automatically with cancellations when a click event
                                 * isn't raised because the page was considered scrolled/drag-scrolled, long-pressed, etc.
                                 *
                                 * This is why we also continuously remove the previous listener, because we cannot be
                                 * certain that it was raised, and therefore cleaned-up.
                                 */
                                if &event.pointer_type() == "touch" {
                                    if let Some(handle_click) = handle_click.read().as_ref() {
                                        _ = owner_document.remove_event_listener_with_callback(
                                            intern("click"),
                                            handle_click.as_ref().unchecked_ref(),
                                        );
                                    }

                                    let closure = Closure::<dyn FnOnce()>::once(
                                        handle_and_dispatch_pointer_down_outside_event,
                                    )
                                    .into_js_value();
                                    handle_click.set(Some(SendWrapper::new(closure.clone())));

                                    let options = AddEventListenerOptions::new();
                                    options.set_once(true);
                                    _ = owner_document
                                        .add_event_listener_with_callback_and_add_event_listener_options(
                                            intern("click"),
                                            closure.as_ref().unchecked_ref(),
                                            &options,
                                        );
                                }
                            } else {
                                // We need to remove the event listener in case the outside click has been canceled.
                                // See: https://github.com/radix-ui/primitives/issues/2171
                                if let Some(handle_click) = handle_click.read().as_ref() {
                                    _ = owner_document.remove_event_listener_with_callback(
                                        intern("click"),
                                        handle_click.as_ref().unchecked_ref(),
                                    );
                                }
                            }

                            is_pointer_inside_react_tree
                                .store(false, std::sync::atomic::Ordering::Relaxed);
                        }
                    })
                    .into_js_value(),
                );

                /*
                 * if this hook executes in a component that mounts via a `pointerdown` event, the event
                 * would bubble up to the document and trigger a `pointerDownOutside` event. We avoid
                 * this by delaying the event listener registration on the document.
                 * This is not React specific, but rather how the DOM works, ie:
                 * ```
                 * button.addEventListener('pointerdown', () => {
                 *   console.log('I will log');
                 *   document.addEventListener('pointerdown', () => {
                 *     console.log('I will also log');
                 *   })
                 * });
                 */
                let timeout = set_timeout_with_handle(
                    {
                        let owner_document = owner_document.clone();
                        let handle_pointer_down = handle_pointer_down.clone();
                        move || {
                            _ = owner_document.add_event_listener_with_callback(
                                intern("pointerdown"),
                                handle_pointer_down.as_ref().unchecked_ref(),
                            );
                        }
                    },
                    Duration::ZERO,
                )
                .unwrap();

                on_cleanup({
                    let handle_click = handle_click.clone();
                    let owner_document = SendWrapper::new(owner_document);
                    let node = SendWrapper::new(node);

                    move || {
                        timeout.clear();
                        _ = owner_document.remove_event_listener_with_callback(
                            intern("pointerdown"),
                            handle_pointer_down.as_ref().unchecked_ref(),
                        );
                        if let Some(handle_click) = handle_click.read().as_ref() {
                            _ = owner_document.remove_event_listener_with_callback(
                                intern("click"),
                                handle_click.as_ref().unchecked_ref(),
                            );
                        }

                        let options = EventListenerOptions::new();
                        options.set_capture(true);
                        _ = node.remove_event_listener_with_callback_and_event_listener_options(
                            intern("pointerdown"),
                            capture_on_pointer_down.as_ref().unchecked_ref(),
                            &options,
                        );
                    }
                });
            },
            false,
        );
    });
}

/// Listens for when focus happens outside a subtree.
fn use_focus_outside(on_focus_outside: MaybeCallback<FocusOutsideEvent>, node: AnyNodeRef) {
    ClientOnly::new(move || {
        let handle_focus_outside =
            Closure::<dyn Fn(FocusOutsideEvent)>::new(move |event: FocusOutsideEvent| {
                #[cfg(debug_assertions)]
                let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                if let Some(ref cb) = on_focus_outside.0.as_ref() {
                    cb.run(event);
                }
            })
            .into_js_value();

        let is_focus_inside_react_tree = Arc::new(AtomicBool::new(false));

        Effect::watch(
            move || node.get(),
            move |node, _, _| {
                let Some(node) = node.clone() else {
                    return;
                };

                let Some(owner_document) = node.owner_document() else {
                    return;
                };

                let capture_on_focusin = SendWrapper::new(
                    Closure::<dyn Fn()>::new({
                        let is_focus_inside_react_tree = Arc::clone(&is_focus_inside_react_tree);
                        move || {
                            is_focus_inside_react_tree
                                .store(true, std::sync::atomic::Ordering::Relaxed);
                        }
                    })
                    .into_js_value(),
                );

                let capture_on_focusout = SendWrapper::new(
                    Closure::<dyn Fn()>::new({
                        let is_focus_inside_react_tree = Arc::clone(&is_focus_inside_react_tree);
                        move || {
                            is_focus_inside_react_tree
                                .store(false, std::sync::atomic::Ordering::Relaxed);
                        }
                    })
                    .into_js_value(),
                );

                {
                    let options = AddEventListenerOptions::new();
                    options.set_capture(true);
                    _ = node.add_event_listener_with_callback_and_add_event_listener_options(
                        intern("focusin"),
                        capture_on_focusin.as_ref().unchecked_ref(),
                        &options,
                    );
                    _ = node.add_event_listener_with_callback_and_add_event_listener_options(
                        intern("focusout"),
                        capture_on_focusout.as_ref().unchecked_ref(),
                        &options,
                    );
                }

                let handle_focus = SendWrapper::new(
                    Closure::<dyn Fn(FocusEvent)>::new({
                        let handle_focus_outside = handle_focus_outside.clone();
                        let is_focus_inside_react_tree = Arc::clone(&is_focus_inside_react_tree);

                        move |event: FocusEvent| {
                            if event.target().is_some()
                                && !is_focus_inside_react_tree
                                    .load(std::sync::atomic::Ordering::Relaxed)
                            {
                                let event_detail = FocusOutsideEventDetail {
                                    original_event: event,
                                };
                                handle_and_dispatch_custom_event(
                                    strings::focus_outside(),
                                    Some(&handle_focus_outside),
                                    event_detail,
                                );
                            }
                        }
                    })
                    .into_js_value(),
                );

                _ = owner_document.add_event_listener_with_callback(
                    intern("focusin"),
                    handle_focus.as_ref().unchecked_ref(),
                );

                on_cleanup({
                    let owner_document = SendWrapper::new(owner_document);
                    let node = SendWrapper::new(node);

                    move || {
                        _ = owner_document.remove_event_listener_with_callback(
                            intern("focusin"),
                            handle_focus.as_ref().unchecked_ref(),
                        );

                        let options = EventListenerOptions::new();
                        options.set_capture(true);
                        _ = node.remove_event_listener_with_callback_and_event_listener_options(
                            intern("focusin"),
                            capture_on_focusin.as_ref().unchecked_ref(),
                            &options,
                        );
                        _ = node.remove_event_listener_with_callback_and_event_listener_options(
                            intern("focusout"),
                            capture_on_focusout.as_ref().unchecked_ref(),
                            &options,
                        );
                    }
                });
            },
            false,
        );
    });
}

fn handle_and_dispatch_custom_event<E: DismissableLayerEventDetail>(
    name: &'static str,
    handler: Option<&JsValue>,
    detail: E,
) {
    let target = detail.original_event().target().unwrap();
    let event = {
        let init = CustomEventInit::new();
        init.set_bubbles(false);
        init.set_cancelable(true);
        init.set_detail(&serde_wasm_bindgen::to_value(&detail).unwrap());
        CustomEvent::new_with_event_init_dict(name, &init).unwrap()
    };

    if let Some(handler) = handler {
        let options = AddEventListenerOptions::new();
        options.set_once(true);
        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                name,
                handler.as_ref().unchecked_ref(),
                &options,
            )
            .unwrap();
    }

    target.dispatch_event(&event.unchecked_ref()).unwrap();
}

struct ClientOnly {
    #[allow(dead_code)]
    effect: Effect<LocalStorage>,
}

impl ClientOnly {
    pub fn new(mut handler: impl FnMut() + 'static) -> Self {
        let effect = Effect::watch(move || (), move |(), _, _| handler(), true);

        Self { effect }
    }
}
