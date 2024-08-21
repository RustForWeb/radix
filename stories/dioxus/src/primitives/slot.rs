use dioxus::prelude::*;
use radix_dioxus_slot::*;

#[component]
pub fn WithoutSlottable() -> Element {
    rsx! {
        SlotWithoutSlottable {
            b {
                "data-slot-element": "",
                "hello"
            }
        }
    }
}

#[component]
pub fn WithSlottable() -> Element {
    rsx! {
        SlotWithSlottable {
            b {
                "data-slot-element": "",
                "hello"
            }
        }
    }
}

#[component]
pub fn WithComposedEvents() -> Element {
    // TODO
    rsx! {}
}

#[component]
pub fn ButtonAsLink() -> Element {
    // TODO
    rsx! {}
}

#[component]
pub fn Chromatic() -> Element {
    // TODO
    rsx! {}
}

#[derive(Clone, PartialEq, Props)]
struct SlotWithoutSlottableProps {
    children: Element,
}

#[component]
fn SlotWithoutSlottable(props: SlotWithSlottableProps) -> Element {
    // TODO: pass attributes to slot
    // TODO: class attribute
    rsx! {
        Slot {
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props)]
struct SlotWithSlottableProps {
    children: Element,
}

#[component]
fn SlotWithSlottable(props: SlotWithSlottableProps) -> Element {
    // TODO: pass attributes to slot
    rsx! {
        Slot {
            Slottable {
                {props.children}
            }
            span {
                "world"
            }
        }
    }
}
