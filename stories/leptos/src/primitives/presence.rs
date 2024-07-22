use leptos::*;
use radix_leptos_presence::*;

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
    view! {}
}

#[component]
pub fn WithUnmountAnimation() -> impl IntoView {
    view! {}
}

#[component]
pub fn WithMultipleMountAnimations() -> impl IntoView {
    view! {}
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
fn Animation() -> impl IntoView {
    let (open, _set_open) = create_signal(false);

    view! {
        <Toggles />
        <Presence present=open>
            <div>Content</div>
        </Presence>
    }
}

#[component]
fn Toggles() -> impl IntoView {
    // TODO
    view! {}
}
