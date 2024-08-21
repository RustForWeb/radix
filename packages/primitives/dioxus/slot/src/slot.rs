use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SlotProps {
    children: Element,
}

#[component]
pub fn Slot(props: SlotProps) -> Element {
    let slottable = false;

    if slottable {
        todo!("Slottable as child of Slot")
        // rsx! {
        //     SlotClone {
        //         {props.children}
        //     }
        // }
    } else {
        rsx! {
            SlotClone {
                {props.children}
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SlotCloneProps {
    children: Element,
}
#[component]
fn SlotClone(props: SlotCloneProps) -> Element {
    log::info!("CHILDREN {:#?}", props.children);

    props.children.and_then(|children| {
        let template = children.template.get();
        if template.roots.len() > 1 {
            None
        } else {
            template.roots.first().and_then(|node| {
                log::info!("SINGLE NODE {:#?}", node);

                rsx! {}

                // match node {
                //     TemplateNode::Element {
                //         tag,
                //         namespace,
                //         attrs,
                //         children,
                //     } => todo!("element"),
                //     TemplateNode::Text { text } => todo!("text"),
                //     TemplateNode::Dynamic { id } => todo!("dynamic"),
                //     TemplateNode::DynamicText { id } => todo!("dynamic text"),
                // }
            })
        }
    })
}

#[derive(Clone, PartialEq, Props)]
pub struct SlottableProps {
    children: Element,
}

#[component]
pub fn Slottable(props: SlottableProps) -> Element {
    props.children
}

// fn map_children(node: VNode) {
//     let template = node.template.get();

//     for root in template.roots {
//         match root {
//             TemplateNode::Element {
//                 tag,
//                 namespace,
//                 attrs,
//                 children,
//             } => todo!(),
//             TemplateNode::Text { text } => todo!(),
//             TemplateNode::Dynamic { id } => todo!(),
//             TemplateNode::DynamicText { id } => todo!(),
//         }
//     }
// }
