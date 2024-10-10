use radix_yew_label::*;
use tailwind_fuse::*;
use web_sys::window;
use yew::prelude::*;

#[function_component]
pub fn Styled() -> Html {
    let root_class = use_memo((), |_| RootClass::default().to_class());

    html! {
        <Label class={(*root_class).clone()}>{"Label"}</Label>
    }
}

#[function_component]
pub fn WithControl() -> Html {
    let control_class = use_memo((), |_| ControlClass::default().to_class());

    html! {
        <>
            <h1>{"Wrapping control"}</h1>
            <Label>
                <Control class={(*control_class).clone()} />{" Label"}
            </Label>

            <h1>{"Referencing control"}</h1>
            <Control id="control" class={(*control_class).clone()} />
            <Label r#for="control">{"Label"}</Label>
        </>
    }
}

#[function_component]
pub fn WithInputNumber() -> Html {
    html! {
        <Label>
            <span>{"Name:"}</span>
            <input type="number" />
        </Label>
    }
}

#[derive(PartialEq, Properties)]
struct ControlProps {
    #[prop_or_default]
    id: Option<String>,
    #[prop_or_default]
    class: Option<String>,
}

#[function_component]
fn Control(props: &ControlProps) -> Html {
    let handle_click = use_callback((), |_, _| {
        window()
            .expect("Window should exist.")
            .alert_with_message("clicked")
            .expect("Alert should be successful.")
    });

    html! {
        <button id={props.id.clone()} class={props.class.clone()} onclick={handle_click}>
            {"Control"}
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
