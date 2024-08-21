use radix_yew_slot::*;
use yew::prelude::*;
use yew_attrs::{attrs, Attrs};

#[function_component]
pub fn WithoutSlottable() -> Html {
    html! {
        <SlotWithoutSlottable>
            <b data-slot-element="">{"hello"}</b>
        </SlotWithoutSlottable>
    }
}

#[function_component]
pub fn WithSlottable() -> Html {
    html! {
        <SlotWithSlottable>
            <b data-slot-element="">{"hello"}</b>
        </SlotWithSlottable>
    }
}

#[function_component]
pub fn WithComposedEvents() -> Html {
    // TODO
    html! {}
}

#[function_component]
pub fn ButtonAsLink() -> Html {
    // TODO
    html! {}
}

#[function_component]
pub fn Chromatic() -> Html {
    // TODO
    html! {}
}

#[derive(PartialEq, Properties)]
struct SlotWithoutSlottableProps {
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SlotWithoutSlottable(props: &SlotWithoutSlottableProps) -> Html {
    let attrs = props
        .attrs
        .clone()
        .merge(attrs! {class="test"})
        .expect("Attrs should be merged.");

    html! {
        <Slot attrs={attrs}>
            {props.children.clone()}
        </Slot>
    }
}

#[derive(PartialEq, Properties)]
struct SlotWithSlottableProps {
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SlotWithSlottable(props: &SlotWithSlottableProps) -> Html {
    html! {
        <Slot attrs={props.attrs.clone()}>
            <Slottable>{props.children.clone()}</Slottable>
            <span>{"world"}</span>
        </Slot>
    }
}
