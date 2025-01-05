use leptos::prelude::*;
use radix_leptos_accessible_icon::*;

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <button type="button">
            <AccessibleIcon
                label="Close"
                render=|attrs| view! {
                    <CrossIcon {..attrs} />
                }
            />
        </button>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <p>
            Some text with an inline accessible icon{" "}
            <AccessibleIcon
                label="Close"
                render=|attrs| view! {
                    <CrossIcon {..attrs} attr:class="inline-block" />
                }
            />
        </p>
    }
}

#[component]
fn CrossIcon() -> impl IntoView {
    view! {
        <svg viewBox="0 0 32 32" width=24 height=24 fill="none" stroke="currentColor">
            <path d="M2 30 L30 2 M30 30 L2 2" />
        </svg>
    }
}
