use std::rc::Rc;

use web_sys::{
    wasm_bindgen::{JsCast, closure::Closure},
    window,
};
use yew::prelude::*;

use crate::use_state_machine::{MachineState, use_state_machine};

#[derive(PartialEq, Properties)]
pub struct PresenceProps {
    pub present: bool,

    #[prop_or_default]
    pub node_ref: NodeRef,
    pub as_child: Callback<PresenceChildProps, Html>,
}

#[derive(Clone, PartialEq)]
pub struct PresenceChildProps {
    pub node_ref: NodeRef,
}

#[function_component]
pub fn Presence(props: &PresenceProps) -> Html {
    let presence = use_presence(props.present);
    let composed_ref = use_composed_ref(&[presence.r#ref, props.node_ref.clone()]);

    let child_props = PresenceChildProps {
        node_ref: composed_ref,
    };

    html! {
        if presence.is_present {
            {props.as_child.emit(child_props)}
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum PresenceState {
    Mounted,
    UnmountSuspended,
    Unmounted,
}

impl MachineState<PresenceEvent> for PresenceState {
    fn next(&self, event: PresenceEvent) -> Option<Self> {
        match self {
            PresenceState::Mounted => match event {
                PresenceEvent::AnimationOut => Some(PresenceState::UnmountSuspended),
                PresenceEvent::Unmount => Some(PresenceState::Unmounted),
                _ => None,
            },
            PresenceState::UnmountSuspended => match event {
                PresenceEvent::Mount => Some(PresenceState::Mounted),
                PresenceEvent::AnimationEnd => Some(PresenceState::Unmounted),
                _ => None,
            },
            PresenceState::Unmounted => match event {
                PresenceEvent::Mount => Some(PresenceState::Mounted),
                _ => None,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum PresenceEvent {
    Mount,
    AnimationOut,
    AnimationEnd,
    Unmount,
}

struct UsePresenceReturn {
    is_present: bool,
    r#ref: NodeRef,
}

#[hook]
fn use_presence(present: bool) -> UsePresenceReturn {
    let node_ref = use_node_ref();
    let styles_ref = use_mut_ref(|| None::<web_sys::CssStyleDeclaration>);
    let prev_present_ref = use_mut_ref(|| present);
    let prev_animation_name_ref = use_mut_ref(|| "none".to_owned());
    let initial_state = if present {
        PresenceState::Mounted
    } else {
        PresenceState::Unmounted
    };
    let state = use_state_machine(initial_state);

    use_effect_with(state.clone(), {
        let styles_ref = styles_ref.clone();
        let prev_animation_name_ref = prev_animation_name_ref.clone();

        move |state| {
            let current_animation_name = get_animation_name(styles_ref.borrow().as_ref());
            *prev_animation_name_ref.borrow_mut() = match ***state {
                PresenceState::Mounted => current_animation_name,
                _ => "none".to_owned(),
            };
        }
    });

    use_effect_with(present, {
        let styles_ref = styles_ref.clone();
        let prev_present_ref = prev_present_ref.clone();
        let prev_animation_name_ref = prev_animation_name_ref.clone();
        let state = state.clone();

        move |present| {
            let present = *present;
            let styles = styles_ref.borrow();
            let was_present = *prev_present_ref.borrow();
            let has_present_changed = was_present != present;

            if has_present_changed {
                let prev_animation_name = prev_animation_name_ref.borrow().clone();
                let current_animation_name = get_animation_name(styles.as_ref());

                if present {
                    state.dispatch(PresenceEvent::Mount);
                } else if current_animation_name == "none"
                    || styles
                        .as_ref()
                        .and_then(|styles| styles.get_property_value("display").ok())
                        == Some("none".into())
                {
                    // If there is no exit animation or the element is hidden, animations won't run so we unmount instantly.
                    state.dispatch(PresenceEvent::Unmount);
                } else {
                    // When `present` changes to `false`, we check changes to animation-name to
                    // determine whether an animation has started. We chose this approach (reading
                    // computed styles) because there is no `animationrun` event and `animationstart`
                    // fires after `animation-delay` has expired which would be too late.
                    let is_animating = prev_animation_name != current_animation_name;

                    if was_present && is_animating {
                        state.dispatch(PresenceEvent::AnimationOut);
                    } else {
                        state.dispatch(PresenceEvent::Unmount);
                    }
                }

                *prev_present_ref.borrow_mut() = present;
            }
        }
    });

    // Triggering an ANIMATION_OUT during an ANIMATION_IN will fire an `animationcancel`
    // event for ANIMATION_IN after we have entered `unmountSuspended` state. So, we
    // make sure we only trigger ANIMATION_END for the currently active animation.
    let handle_animation_end: Rc<Closure<dyn Fn(AnimationEvent)>> = use_memo((), |_| {
        Closure::new({
            let node_ref = node_ref.clone();
            let styles_ref = styles_ref.clone();
            let state = state.clone();

            move |event: AnimationEvent| {
                let current_animation_name = get_animation_name(styles_ref.borrow().as_ref());
                let is_current_animation = current_animation_name.contains(&event.animation_name());
                if is_current_animation
                    && event.target().as_ref()
                        == node_ref
                            .get()
                            .as_ref()
                            .map(|node| node.unchecked_ref::<web_sys::EventTarget>())
                {
                    state.dispatch(PresenceEvent::AnimationEnd);
                }
            }
        })
    });

    let handle_animation_start: Rc<Closure<dyn Fn(AnimationEvent)>> = use_memo((), |_| {
        Closure::new({
            let node_ref = node_ref.clone();
            let styles_ref = styles_ref.clone();

            move |event: AnimationEvent| {
                if event.target().as_ref()
                    == node_ref
                        .get()
                        .as_ref()
                        .map(|node| node.unchecked_ref::<web_sys::EventTarget>())
                {
                    // If animation occurred, store its name as the previous animation.
                    *prev_animation_name_ref.borrow_mut() =
                        get_animation_name(styles_ref.borrow().as_ref());
                }
            }
        })
    });

    let node = use_state_eq(|| None);
    use_effect({
        let node_ref = node_ref.clone();
        let node = node.clone();

        move || {
            node.set(node_ref.get());

            if let Some(node) = node_ref.cast::<web_sys::Element>() {
                *styles_ref.borrow_mut() = window()
                    .expect("Window should exist.")
                    .get_computed_style(&node)
                    .expect("Element is valid.");
            }
        }
    });

    use_effect_with(node, {
        let state = state.clone();
        let handle_animation_start = handle_animation_start.clone();
        let handle_animation_end = handle_animation_end.clone();

        move |node| {
            let mut cleanup: Option<Box<dyn Fn()>> = None;

            if let Some(node) = (**node).clone() {
                node.add_event_listener_with_callback(
                    "animationstart",
                    (*handle_animation_start).as_ref().unchecked_ref(),
                )
                .expect("Animation start event listener should be added.");
                node.add_event_listener_with_callback(
                    "animationcancel",
                    (*handle_animation_end).as_ref().unchecked_ref(),
                )
                .expect("Animation cancel event listener should be added.");
                node.add_event_listener_with_callback(
                    "animationend",
                    (*handle_animation_end).as_ref().unchecked_ref(),
                )
                .expect("Animation end event listener should be added.");

                cleanup = Some(Box::new(move || {
                    node.remove_event_listener_with_callback(
                        "animationstart",
                        (*handle_animation_start).as_ref().unchecked_ref(),
                    )
                    .expect("Animation start event listener should be removed.");
                    node.remove_event_listener_with_callback(
                        "animationcancel",
                        (*handle_animation_end).as_ref().unchecked_ref(),
                    )
                    .expect("Animation cancel event listener should be removed.");
                    node.remove_event_listener_with_callback(
                        "animationend",
                        (*handle_animation_end).as_ref().unchecked_ref(),
                    )
                    .expect("Animation end event listener should be removed.");
                }));
            } else {
                // Transition to the unmounted state if the node is removed prematurely.
                // We avoid doing so during cleanup as the node may change but still exist.
                state.dispatch(PresenceEvent::AnimationEnd);
            }

            move || {
                if let Some(cleanup) = cleanup {
                    cleanup();
                }
            }
        }
    });

    use_effect_with((), move |_| {
        move || {
            // Move closures to prevent them from being dropped too early.
            drop(handle_animation_start);
            drop(handle_animation_end);
        }
    });

    UsePresenceReturn {
        is_present: [PresenceState::Mounted, PresenceState::UnmountSuspended].contains(&**state),
        r#ref: node_ref,
    }
}

fn get_animation_name(styles: Option<&web_sys::CssStyleDeclaration>) -> String {
    styles
        .and_then(|styles| styles.get_property_value("animation-name").ok())
        .unwrap_or("none".into())
}
