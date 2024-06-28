// TODO: remove
#![allow(dead_code, unused_variables)]

use leptos::{html::AnyElement, *};

#[component]
pub fn Slottable(children: ChildrenFn) -> impl IntoView {
    children
}

#[component]
pub fn Slot(
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <SlotClone node_ref=node_ref attrs=attrs>
            {children().as_children().iter().map(|child| {
                match child {
                    View::Component(component) if component.name() == "Slottable" => {
                        // TODO: copy code from primitive
                        todo!()
                    },
                    _ => child
                }
            }).collect_view()}
        </SlotClone>
    }
}

#[component]
pub fn SlotClone(
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {}
}

fn is_slottable(child: &View) -> bool {
    matches!(child, View::Component(c) if c.name() == "Slottable")
}
