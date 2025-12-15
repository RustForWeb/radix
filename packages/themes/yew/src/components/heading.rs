use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent};
use yew_style::Style;

use crate::{
    components::heading_props::{HeadingAs, HeadingAsProp, HeadingSizeProp},
    helpers::extract_props::extract_props,
    props::{
        color_prop::ColorProp,
        high_contrast_prop::HighContrastProp,
        leading_trim_prop::LeadingTrimProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        text_align_prop::TextAlignProp,
        text_wrap_prop::TextWrapProp,
        truncate_prop::TruncateProp,
        weight_prop::WeightProp,
    },
};

#[derive(PartialEq, Properties)]
pub struct HeadingProps {
    #[prop_or_default]
    pub r#as: HeadingAsProp,
    #[prop_or_default]
    pub size: HeadingSizeProp,
    #[prop_or_default]
    pub weight: WeightProp,
    #[prop_or_default]
    pub align: TextAlignProp,
    #[prop_or_default]
    pub trim: LeadingTrimProp,
    #[prop_or_default]
    pub truncate: TruncateProp,
    #[prop_or_default]
    pub wrap: TextWrapProp,
    #[prop_or_default]
    pub color: ColorProp,
    #[prop_or_default]
    pub high_contrast: HighContrastProp,
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
    pub as_child: Option<Callback<HeadingChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
pub struct HeadingChildProps {
    #[struct_component(dynamic_tag = true)]
    pub r#as: HeadingAs,
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub data_accent_color: Option<String>,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn Heading(props: &HeadingProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.r#as,
            &props.size,
            &props.weight,
            &props.align,
            &props.trim,
            &props.truncate,
            &props.wrap,
            &props.color,
            &props.high_contrast,
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

    let child_props = HeadingChildProps {
        r#as: props.r#as.0,
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-Heading", class).to_string(),
        data_accent_color: props.color.0.map(|color| color.to_string()),
        id: props.id.clone(),
        style,
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
