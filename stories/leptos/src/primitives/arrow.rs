use leptos::prelude::*;
use radix_leptos_arrow::*;

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <Arrow width=20.0 height=10.0 attr:style="vertical-align: middle; fill: crimson;" />
    }
}

#[component]
pub fn CustomSizes() -> impl IntoView {
    view! {
        <Arrow width=40.0 height=10.0 attr:style="vertical-align: middle; display: inline-block;" />
        <Arrow width=50.0 height=30.0 attr:style="vertical-align: middle; display: inline-block;" />
        <Arrow width=20.0 height=100.0 attr:style="vertical-align: middle; display: inline-block;" />
    }
}

#[component]
pub fn CustomArrow() -> impl IntoView {
    view! {
        <Arrow as_child=true>
            <div
                style:width="20px"
                style:height="10px"
                style:border-bottom-left-radius="10px"
                style:border-bottom-right-radius="10px"
                style:background-color="tomato"
            />
        </Arrow>
    }
}
