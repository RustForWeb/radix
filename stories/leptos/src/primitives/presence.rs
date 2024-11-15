use std::ops::Deref;

use html::AnyElement;
use leptos::*;
use radix_leptos_presence::*;
use tailwind_fuse::*;
use web_sys::wasm_bindgen::{closure::Closure, JsCast};

#[component]
pub fn Basic() -> impl IntoView {
    let (open, set_open) = create_signal(false);

    view! {
        <button on:click=move |_| set_open.update(move |open| *open = !*open)>Toggle</button>

        <Presence present=open>
            <div>Content</div>
        </Presence>
    }
}

#[component]
pub fn WithMountAnimation() -> impl IntoView {
    let mount_animation_class = Memo::new(move |_| MountAnimationClass::default().to_class());

    view! {
        <Animation attr:class=mount_animation_class />
    }
}

#[component]
pub fn WithUnmountAnimation() -> impl IntoView {
    let unmount_animation_class = Memo::new(move |_| UnmountAnimationClass::default().to_class());

    view! {
        <Animation attr:class=unmount_animation_class />
    }
}

#[component]
pub fn WithMultipleMountAnimations() -> impl IntoView {
    let multiple_mount_animations_class =
        Memo::new(move |_| MultipleMountAnimationsClass::default().to_class());

    view! {
        <Animation attr:class=multiple_mount_animations_class />
    }
}

#[component]
pub fn WithOpenAndCloseAnimation() -> impl IntoView {
    let open_and_close_animation_class =
        Memo::new(move |_| OpenAndCloseAnimationClass::default().to_class());

    view! {
        <Animation attr:class=open_and_close_animation_class />
    }
}

#[component]
pub fn WithMultipleOpenAndCloseAnimations() -> impl IntoView {
    let multiple_open_and_close_animations_class =
        Memo::new(move |_| MultipleOpenAndCloseAnimationsClass::default().to_class());

    view! {
        <Animation attr:class=multiple_open_and_close_animations_class />
    }
}

#[component]
pub fn WithDeferredMountAnimation() -> impl IntoView {
    let mount_animation_class = Memo::new(move |_| MountAnimationClass::default().to_class());

    let node_ref = NodeRef::new();
    let timer = RwSignal::new(0);
    let (open, set_open) = create_signal(false);
    let (animate, set_animate) = create_signal(false);

    let handler: Closure<dyn Fn()> = Closure::new(move || {
        set_animate.set(true);
    });

    Effect::new(move |_| {
        if open.get() {
            timer.set(
                window()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        handler.as_ref().unchecked_ref(),
                        150,
                    )
                    .expect("Timeout should be set."),
            )
        } else {
            set_animate.set(false);
            window().clear_timeout_with_handle(timer.get());
        }
    });

    on_cleanup(move || {
        window().clear_timeout_with_handle(timer.get());
    });

    view! {
        <p>
            Deferred animation should unmount correctly when toggled. Content will flash briefly while
            we wait for animation to be applied.
        </p>
        <Toggles
            open=open
            on_open_change=move |open| set_open.set(open)
            node_ref=node_ref
        />
        <Presence present=open node_ref=node_ref>
            <div attr:class=move || animate.get().then_some(mount_animation_class.get())>
                Content
            </div>
        </Presence>
    }
}

#[component]
fn Animation(#[prop(attrs)] attrs: Vec<(&'static str, Attribute)>) -> impl IntoView {
    let attrs = StoredValue::new(attrs);

    let node_ref = NodeRef::new();
    let (open, set_open) = create_signal(false);

    view! {
        <Toggles
            open=open
            on_open_change=move |open| set_open.set(open)
            node_ref=node_ref
        />
        <Presence present=open node_ref=node_ref>
            <div {..attrs.get_value()} data-state=move || match open.get() {
                true => "open",
                false => "closed"
            }>
                Content
            </div>
        </Presence>
    }
}

#[component]
fn Toggles(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(into)] node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
    let handle_toggle_visilbity = move |_| {
        if let Some(node) = node_ref.get_untracked() {
            let style = node.deref().style();
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
    };

    view! {
        <form style:display="flex" style:margin-bottom="30px">
            <fieldset>
                <legend>Mount</legend>
                <button type="button" on:click=move |_| on_open_change.call(!open.get())>
                    toggle
                </button>
            </fieldset>
            <fieldset>
                <legend>Visibility (triggers cancel event)</legend>
                <button type="button" on:click=handle_toggle_visilbity>
                    toggle
                </button>
            </fieldset>
        </form>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "animate-[presenceFadeIn_3s_ease-out]")]
struct MountAnimationClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "data-[state=closed]:animate-[presenceFadeOut_3s_ease-out]")]
struct UnmountAnimationClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "animate-[presenceFadeIn_6s_cubic-bezier(0.22,1,0.36,1),presenceSlideUp_6s_cubic-bezier(0.22,1,0.36,1)]"
)]
struct MultipleMountAnimationsClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "data-[state=open]:animate-[presenceFadeIn_3s_ease-out] data-[state=closed]:animate-[presenceFadeOut_3s_ease-out]"
)]
struct OpenAndCloseAnimationClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "data-[state=open]:animate-[presenceFadeIn_6s_cubic-bezier(0.22,1,0.36,1),presenceSlideUp_6s_cubic-bezier(0.22,1,0.36,1)] data-[state=closed]:animate-[presenceFadeOut_6s_cubic-bezier(0.22,1,0.36,1),presenceSlideDown_6s_cubic-bezier(0.22,1,0.36,1)]"
)]
struct MultipleOpenAndCloseAnimationsClass {}
