use std::any::TypeId;

use regex::Regex;
use yew::{prelude::*, virtual_dom::VNode};
use yew_attrs::Attrs;

#[derive(PartialEq, Properties)]
pub struct SlotProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Slot(props: &SlotProps) -> Html {
    let mut children = props.children.clone();
    let children_list = children.to_vlist_mut();
    let slottable = children_list.iter().find(|child| is_slottable(child));

    if let Some(_slottable) = slottable {
        todo!("Slottable as child of Slot")
        // html! {
        //     <SlotClone node_ref={props.node_ref.clone()} attrs={props.attrs.clone()}>
        //         {props.children.clone()}
        //     </SlotClone>
        // }
    } else {
        html! {
            <SlotClone node_ref={props.node_ref.clone()} attrs={props.attrs.clone()}>
                {props.children.clone()}
            </SlotClone>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct SlotCloneProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SlotClone(props: &SlotCloneProps) -> Html {
    match &props.children {
        VNode::VTag(tag) => {
            let attrs = props
                .attrs
                .clone()
                .merge(Attrs::new(
                    props.attrs.attributes.clone(),
                    props.attrs.listeners.clone(),
                ))
                .expect("Attrs should be merged,");

            attrs
                .new_vtag(
                    tag.tag(),
                    props.node_ref.clone(),
                    Default::default(),
                    tag.children().cloned().unwrap_or_default(),
                )
                .into()
        }
        VNode::VComp(_comp) => todo!("component as child of Slot"),
        _ => VNode::default(),
    }
}

#[derive(PartialEq, Properties)]
pub struct SlottableProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Slottable(props: &SlottableProps) -> Html {
    props.children.clone()
}

fn is_slottable(child: &VNode) -> bool {
    match child {
        VNode::VComp(comp) => {
            // Very hacky way to obtain the type ID of the virtual component
            let formatted = format!("{:?}", comp);
            let type_id = Regex::new(r"TypeId\(0x[0-9a-f]+\)")
                .expect("Regex should be valid.")
                .captures(&formatted)
                .and_then(|caps| caps.get(0))
                .map(|m| m.as_str().to_string());

            type_id.is_some_and(|type_id| type_id == format!("{:?}", TypeId::of::<Slottable>()))
        }
        _ => false,
    }
}
