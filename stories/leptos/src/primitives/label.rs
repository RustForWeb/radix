use leptos::ev;
use leptos::prelude::*;
use radix_leptos_label::*;

/// Styled example with a basic label
#[component]
pub fn Basic() -> impl IntoView {
    view! {
        <Label attr:class="inline-block align-middle cursor-default border border-solid border-gray-200 p-2">
            "Basic Label"
        </Label>
    }
}

/// Shows both wrapping and referencing controls
#[component]
pub fn WithControl() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <h2>"Wrapping control"</h2>
            <Label attr:class="inline-flex gap-2">
                <Control /> " Label with wrapped control"
            </Label>

            <h2>"Referencing control"</h2>
            <Control attr:id="control" />
            <Label attr:r#for="control" attr:class="ml-2">"Label referencing control"</Label>
        </div>
    }
}

/// Demonstrates using Label with various form inputs
#[component]
pub fn WithInputs() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <Label attr:class="flex items-center gap-2">
                <span>"Name:"</span>
                <input r#type="text" class="border p-1" />
            </Label>

            <Label attr:class="flex items-center gap-2">
                <span>"Quantity:"</span>
                <input r#type="number" class="border p-1" />
            </Label>
        </div>
    }
}

/// Shows label with custom event handling
#[component]
pub fn WithEvents() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div class="space-y-4">
            <Label
                attr:class="inline-block p-2 border"
                on_mouse_down=move |_: ev::MouseEvent| set_count.update(|mut c| *c = *c + 1)
            >
                <span>"Click counter: "</span>
                <span>{count}</span>
            </Label>
        </div>
    }
}

/// Helper component for the control examples
#[component]
fn Control() -> impl IntoView {
    view! {
        <button
            class="inline-flex border border-solid border-gray-200 p-2 hover:bg-red-100"
            on:click=move |_| window().alert_with_message("clicked").expect("Alert should work")
        >
            "Control"
        </button>
    }
}