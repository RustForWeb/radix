use radix_yew_label::*;
use radix_yew_primitive::*;
use tailwind_fuse::*;
use yew::prelude::*;
use yew_attrs::{attrs, Attrs};

#[function_component]
pub fn Styled() -> Html {
    let root_class = use_memo((), move |_| RootClass::default().to_class());

    html! {
        <Label attrs={attrs! {class={(*root_class).clone()}}}>{"Label"}</Label>
    }
}

#[function_component]
pub fn WithControl() -> Html {
    let control_class = use_memo((), move |_| ControlClass::default().to_class());

    html! {
        <>
            <h1>{"Wrapping control"}</h1>
            <Label>
                <Control attrs={attrs! {class={(*control_class).clone()}}} />{" Label"}
            </Label>

            <h1>{"Referencing control"}</h1>
            <Control attrs={attrs! {id="control" class={(*control_class).clone()}}} />
            <Label attrs={attrs! {for="control"}}>{"Label"}</Label>
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
    attrs: Attrs,
}

#[function_component]
fn Control(_props: &ControlProps) -> Html {
    // TODO: add on click listener to attrs
    // TODO: add attrs once it is clone: attrs={props.attrs.clone()}
    html! {
        <Primitive element="button">
            {"Control"}
        </Primitive>
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
