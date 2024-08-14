use radix_yew_label::*;
use tailwind_fuse::*;
use yew::prelude::*;

#[function_component]
pub fn Styled() -> Html {
    // let root_class = use_memo((), move |_| RootClass::default().to_class());

    html! {
        <Label>{"Label"}</Label>
        // <Label attr:class=root_class>{"Label"}</Label>
    }
}

#[function_component]
pub fn WithControl() -> Html {
    // let control_class = use_memo((), move |_| ControlClass::default().to_class());

    html! {
        // <h1>{"Wrapping control"}</h1>
        // <Label>
        //     <Control attr:class=control_class /> Label
        // </Label>

        // <h1>{"Referencing control"}</h1>
        // <Control attr:id="control" attr:class=control_class />
        // <Label attr:for="control">{"Label"}</Label>
    }
}

#[function_component]
pub fn WithInputNumber() -> Html {
    html! {
        // <Label>
        //     <span>{"Name:"}</span>
        //     <input type="number" />
        // </Label>
    }
}

// #[derive(PartialEq, Properties)]
// struct ControlProps {}

// #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>

#[function_component]
// fn Control(_props: &ControlProps) -> Html {
fn Control() -> Html {
    html! {
        // <button {..attrs} on:click=move |_| window().alert_with_message("clicked").expect("Alert should be successful.")>
        //     {"Control"}
        // </button>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "inline-block align-middle cursor-default border-[1px] border-solid border-[gainsboro] p-[10px]"
)]
pub struct RootClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "inline-flex border-[1px] border-solid border-[gainsboro] p-[10px] align-middle m-[0px_10px] hover:bg-[red]"
)]
pub struct ControlClass {}
