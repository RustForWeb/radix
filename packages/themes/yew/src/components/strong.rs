use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};

use crate::{
    helpers::{extract_props::extract_props, merge_styles::Style},
    props::{text_wrap_prop::TextWrapProp, truncate_prop::TruncateProp},
};

#[derive(PartialEq, Properties)]
pub struct StrongProps {
    #[prop_or_default]
    pub truncate: TruncateProp,
    #[prop_or_default]
    pub wrap: TextWrapProp,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<StrongChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "strong")]
pub struct StrongChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: String,
}

#[function_component]
pub fn Strong(props: &StrongProps) -> Html {
    let (class, style) = extract_props(
        &[&props.truncate, &props.wrap],
        props.class.clone(),
        props.style.clone().into(),
    );

    let child_props = StrongChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-Strong", class).to_string(),
        id: props.id.clone(),
        style: style.to_string(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
