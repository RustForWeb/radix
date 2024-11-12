use std::rc::Rc;

use radix_yew_presence::*;
use tailwind_fuse::*;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    window,
};
use yew::prelude::*;

#[function_component]
pub fn Basic() -> Html {
    let open = use_state_eq(|| false);

    let handle_click = use_callback(open.clone(), |_, open| open.set(!**open));

    html! {
        <>
            <button onclick={handle_click}>{"Toggle"}</button>

            <Presence
                present={*open}

                as_child={Callback::from(|PresenceChildProps { node_ref }| html! {
                    <div ref={node_ref}>{"Content"}</div>
                })}
            />
        </>
    }
}

#[function_component]
pub fn WithMountAnimation() -> Html {
    let mount_animation_class = use_memo((), |_| MountAnimationClass::default().to_class());

    html! {
        <Animation class={(*mount_animation_class).clone()} />
    }
}

#[function_component]
pub fn WithUnmountAnimation() -> Html {
    let unmount_animation_class = use_memo((), |_| UnmountAnimationClass::default().to_class());

    html! {
        <Animation class={(*unmount_animation_class).clone()} />
    }
}

#[function_component]
pub fn WithMultipleMountAnimations() -> Html {
    let multiple_mount_animations_class =
        use_memo((), |_| MultipleMountAnimationsClass::default().to_class());

    html! {
        <Animation class={(*multiple_mount_animations_class).clone()} />
    }
}

#[function_component]
pub fn WithOpenAndCloseAnimation() -> Html {
    let open_and_close_animation_class =
        use_memo((), |_| OpenAndCloseAnimationClass::default().to_class());

    html! {
        <Animation class={(*open_and_close_animation_class).clone()} />
    }
}

#[function_component]
pub fn WithMultipleOpenAndCloseAnimations() -> Html {
    let multiple_open_and_close_animations_class = use_memo((), |_| {
        MultipleOpenAndCloseAnimationsClass::default().to_class()
    });

    html! {
        <Animation class={(*multiple_open_and_close_animations_class).clone()} />
    }
}

#[function_component]
pub fn WithDeferredMountAnimation() -> Html {
    let mount_animation_class = use_memo((), |_| MountAnimationClass::default().to_class());

    let node_ref = use_node_ref();
    let timer_ref = use_mut_ref(|| 0);
    let open = use_state_eq(|| false);
    let animate = use_state_eq(|| false);

    let handle_open_change = use_callback(open.clone(), |value, open| open.set(value));

    let closure: Rc<Closure<dyn Fn()>> = use_memo((), |_| {
        Closure::new({
            let animate = animate.clone();

            move || {
                animate.set(true);
            }
        })
    });

    use_effect_with(open.clone(), {
        let timer_ref = timer_ref.clone();
        let animate = animate.clone();
        let closure = closure.clone();

        move |open| {
            let window = window().expect("Window should exist.");

            if **open {
                *timer_ref.borrow_mut() = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        (*closure).as_ref().unchecked_ref(),
                        150,
                    )
                    .expect("Timeout should be set.");
            } else {
                animate.set(false);
                window.clear_timeout_with_handle(*timer_ref.borrow());
            }
        }
    });

    use_effect_with((), move |_| {
        move || {
            window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(*timer_ref.borrow());

            // Move closure to prevent it from being dropped too early.
            drop(closure);
        }
    });

    html! {
        <>
            <p>
                {"Deferred animation should unmount correctly when toggled. Content will flash briefly while
                we wait for animation to be applied."}
            </p>
            <Toggles
                open={*open}
                on_open_change={handle_open_change}
                node_ref={node_ref.clone()}
            />
            <Presence
                present={*open}

                node_ref={node_ref}
                as_child={Callback::from({
                    let animate = animate.clone();

                    move |PresenceChildProps { node_ref }| html! {
                        <div ref={node_ref} class={animate.then_some((*mount_animation_class).clone())}>
                            {"Content"}
                        </div>
                    }
                })}
            />
        </>
    }
}

#[derive(PartialEq, Properties)]
struct AnimationProps {
    class: String,
}

#[function_component]
fn Animation(props: &AnimationProps) -> Html {
    let node_ref = use_node_ref();
    let open = use_state_eq(|| false);

    let handle_open_change = use_callback(open.clone(), |value, open| open.set(value));

    html! {
        <>
            <Toggles
                open={*open}
                on_open_change={handle_open_change}
                node_ref={node_ref.clone()}
            />
            <Presence
                present={*open}

                node_ref={node_ref}
                as_child={Callback::from({
                    let class = props.class.clone();

                    move |PresenceChildProps { node_ref }| html! {
                        <div
                            ref={node_ref}
                            class={class.clone()}
                            data-state={match *open {
                                true => "open",
                                false => "closed"
                            }}
                        >
                            {"Content"}
                        </div>
                    }
                })}
            />
        </>
    }
}

#[derive(PartialEq, Properties)]
struct TogglesProps {
    open: bool,
    on_open_change: Callback<bool>,
    node_ref: NodeRef,
}

#[function_component]
fn Toggles(props: &TogglesProps) -> Html {
    let handle_toggle = use_callback(
        (props.open, props.on_open_change.clone()),
        |_, (open, on_open_change)| on_open_change.emit(!*open),
    );

    let handle_toggle_visilbity = use_callback(props.node_ref.clone(), |_, node_ref| {
        if let Some(node) = node_ref.cast::<web_sys::HtmlElement>() {
            let style = node.style();
            if style.get_property_value("display").ok() == Some("none".into()) {
                style
                    .set_property("display", "block")
                    .expect("Style should be updated.");
            } else {
                style
                    .set_property("display", "none")
                    .expect("Style should be updated.");
            }
        }
    });

    html! {
        <form style="display: flex; margin-bottom: 30px;">
            <fieldset>
                <legend>{"Mount"}</legend>
                <button type="button" onclick={handle_toggle}>
                    {"toggle"}
                </button>
            </fieldset>
            <fieldset>
                <legend>{"Visibility (triggers cancel event)"}</legend>
                <button type="button" onclick={handle_toggle_visilbity}>
                    {"toggle"}
                </button>
            </fieldset>
        </form>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "animate-[presenceFadeIn_3s_ease-out]")]
pub struct MountAnimationClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "data-[state=closed]:animate-[presenceFadeOut_3s_ease-out]")]
pub struct UnmountAnimationClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "animate-[presenceFadeIn_6s_cubic-bezier(0.22,1,0.36,1),presenceSlideUp_6s_cubic-bezier(0.22,1,0.36,1)]"
)]
pub struct MultipleMountAnimationsClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "data-[state=open]:animate-[presenceFadeIn_3s_ease-out] data-[state=closed]:animate-[presenceFadeOut_3s_ease-out]"
)]
pub struct OpenAndCloseAnimationClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "data-[state=open]:animate-[presenceFadeIn_6s_cubic-bezier(0.22,1,0.36,1),presenceSlideUp_6s_cubic-bezier(0.22,1,0.36,1)] data-[state=closed]:animate-[presenceFadeOut_6s_cubic-bezier(0.22,1,0.36,1),presenceSlideDown_6s_cubic-bezier(0.22,1,0.36,1)]"
)]
pub struct MultipleOpenAndCloseAnimationsClass {}
