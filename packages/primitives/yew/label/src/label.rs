use radix_yew_primitive::Primitive;
use yew::prelude::*;
use yew_attrs::Attrs;

#[derive(PartialEq, Properties)]
pub struct LabelProps {
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
pub fn Label(props: &LabelProps) -> Html {
    html! {
        <Primitive
            element="label"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={props.attrs.clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}
