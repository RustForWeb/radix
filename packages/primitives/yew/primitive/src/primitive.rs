use yew::{
    prelude::*,
    virtual_dom::{Attributes, Listeners, VTag},
};

#[derive(PartialEq, Properties)]
pub struct PrimitiveProps {
    pub element: String,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    pub attrs: Attributes,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Primitive(props: &PrimitiveProps) -> Html {
    // let test = html! {
    //     <@{props.element.clone()} ref={props.node_ref.clone()}>
    //         {props.children.clone()}
    //     </@>
    // };

    let a = VTag::__new_other(
        props.element.clone().into(),
        props.node_ref.clone(),
        None,
        props.attrs.clone(),
        Listeners::None,
        props.children.clone(),
    );

    // log::info!("{:?}", test);
    log::info!("{:?}", a);

    a.into()
}
