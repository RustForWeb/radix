use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent, struct_component};
use yew_style::Style;

use crate::{
    components::inset_props::{
        InsetClipProp, InsetPProp, InsetPbProp, InsetPlProp, InsetPrProp, InsetPtProp, InsetPxProp,
        InsetPyProp, InsetSideProp,
    },
    helpers::extract_props::extract_props,
    props::margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
};

#[derive(PartialEq, Properties)]
pub struct InsetProps {
    #[prop_or_default]
    pub side: InsetSideProp,
    #[prop_or_default]
    pub clip: InsetClipProp,
    #[prop_or_default]
    pub p: InsetPProp,
    #[prop_or_default]
    pub px: InsetPxProp,
    #[prop_or_default]
    pub py: InsetPyProp,
    #[prop_or_default]
    pub pt: InsetPtProp,
    #[prop_or_default]
    pub pr: InsetPrProp,
    #[prop_or_default]
    pub pb: InsetPbProp,
    #[prop_or_default]
    pub pl: InsetPlProp,
    #[prop_or_default]
    pub m: MProp,
    #[prop_or_default]
    pub mx: MxProp,
    #[prop_or_default]
    pub my: MyProp,
    #[prop_or_default]
    pub mt: MtProp,
    #[prop_or_default]
    pub mr: MrProp,
    #[prop_or_default]
    pub mb: MbProp,
    #[prop_or_default]
    pub ml: MlProp,

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
    pub as_child: Option<Callback<InsetChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct InsetChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn Inset(props: &InsetProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.side,
            &props.clip,
            &props.p,
            &props.px,
            &props.py,
            &props.pt,
            &props.pr,
            &props.pb,
            &props.pl,
            &props.m,
            &props.mx,
            &props.my,
            &props.mt,
            &props.mr,
            &props.mb,
            &props.ml,
        ],
        props.class.clone(),
        props.style.clone().into(),
    );

    let child_props = InsetChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-Inset", class).to_string(),
        id: props.id.clone(),
        style,
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
