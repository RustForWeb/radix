use std::ops::Deref;

use html::AnyElement;
use leptos::*;
use radix_leptos_presence::*;
use tailwind_fuse::*;

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
    let multiple_mount_animation_class =
        Memo::new(move |_| MultipleMountAnimationsClass::default().to_class());

    view! {
        <Animation attr:class=multiple_mount_animation_class />
    }
}

#[component]
pub fn WithOpenAndCloseAnimation() -> impl IntoView {
    view! {}
}

#[component]
pub fn WithMultipleOpenAndCloseAnimations() -> impl IntoView {
    view! {}
}

#[component]
pub fn WithDeferredMountAnimation() -> impl IntoView {
    view! {}
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
pub struct MountAnimationClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "data-[state=closed]:animate-[presenceFadeOut_3s_ease-out]")]
pub struct UnmountAnimationClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "animate-[presenceFadeIn_6s_cubic-bezier(0.22,1,0.36,1),presenceSlideUp_6s_cubic-bezier(0.22,1,0.36,1)]"
)]
pub struct MultipleMountAnimationsClass {}
