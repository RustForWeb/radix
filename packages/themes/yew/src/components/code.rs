use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};

use crate::{
    components::code_props::{CodeSizeProp, CodeVariantProp},
    helpers::{extract_props::extract_props, merge_styles::Style},
    props::{
        color_prop::AccentColorProp,
        high_contrast_prop::HighContrastProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        text_wrap_prop::TextWrapProp,
        truncate_prop::TruncateProp,
        weight_prop::WeightProp,
    },
};
#[derive(PartialEq, Properties)]
pub struct CodeProps {
    #[prop_or_default]
    pub size: CodeSizeProp,
    #[prop_or_default]
    pub variant: CodeVariantProp,
    #[prop_or_default]
    pub weight: WeightProp,
    #[prop_or_default]
    pub color: AccentColorProp,
    #[prop_or_default]
    pub high_contrast: HighContrastProp,
    #[prop_or_default]
    pub truncate: TruncateProp,
    #[prop_or_default]
    pub wrap: TextWrapProp,
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
    pub as_child: Option<Callback<CodeChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "code")]
pub struct CodeChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub data_accent_color: String,
    pub id: Option<String>,
    pub style: String,
}

#[function_component]
pub fn Code(props: &CodeProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.size,
            &props.variant,
            &props.weight,
            &props.color,
            &props.high_contrast,
            &props.truncate,
            &props.wrap,
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

    let child_props = CodeChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-reset", "rt-Code", class).to_string(),
        data_accent_color: props
            .color
            .0
            .map(|color| color.to_string())
            .unwrap_or("".to_owned()),
        id: props.id.clone(),
        style: style.to_string(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
