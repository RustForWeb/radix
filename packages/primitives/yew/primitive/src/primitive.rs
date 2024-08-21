use radix_yew_slot::Slot;
use yew::prelude::*;
use yew_attrs::Attrs;

#[derive(PartialEq, Properties)]
pub struct PrimitiveProps {
    pub element: String,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Primitive(props: &PrimitiveProps) -> Html {
    if props.as_child {
        html! {
            <Slot node_ref={props.node_ref.clone()} attrs={props.attrs.clone()}>
                {props.children.clone()}
            </Slot>
        }
    } else {
        props
            .attrs
            .clone()
            .new_vtag(
                &props.element,
                props.node_ref.clone(),
                Default::default(),
                props.children.clone(),
            )
            .into()
    }
}
