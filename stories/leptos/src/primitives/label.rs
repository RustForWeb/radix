use leptos::prelude::*;
use radix_leptos_label::*;
use tailwind_fuse::*;

#[component]
pub fn Styled() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());

    view! {
        <Label attr:class=root_class>Label</Label>
    }
}

#[component]
pub fn WithControl() -> impl IntoView {
    let control_class = Memo::new(move |_| ControlClass::default().to_class());

    view! {
        <h1>Wrapping control</h1>
        <Label>
            <Control attr:class=control_class /> Label
        </Label>

        <h1>Referencing control</h1>
        <Control attr:id="control" attr:class=control_class />
        <Label attr:r#for="control">Label</Label>
    }
}

#[component]
pub fn WithInputNumber() -> impl IntoView {
    view! {
        <Label>
            <span>Name:</span>
            <input type="number" />
        </Label>
    }
}

#[component]
fn Control() -> impl IntoView {
    view! {
        <button on:click=move |_| window().alert_with_message("clicked").expect("Alert should be successful.")>
            Control
        </button>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "inline-block align-middle cursor-default border-[1px] border-solid border-[gainsboro] p-[10px]"
)]
struct RootClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "inline-flex border-[1px] border-solid border-[gainsboro] p-[10px] align-middle m-[0px_10px] hover:bg-[red]"
)]
struct ControlClass {}
