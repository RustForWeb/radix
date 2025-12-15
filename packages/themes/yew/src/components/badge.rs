use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent};
use yew_style::Style;

use crate::{
    components::badge_props::{BadgeSizeProp, BadgeVariantProp},
    helpers::extract_props::extract_props,
    props::{
        color_prop::AccentColorProp,
        high_contrast_prop::HighContrastProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        radius_prop::RadiusProp,
    },
};

#[derive(PartialEq, Properties)]
pub struct BadgeProps {
    #[prop_or_default]
    pub size: BadgeSizeProp,
    #[prop_or_default]
    pub variant: BadgeVariantProp,
    #[prop_or_default]
    pub color: AccentColorProp,
    #[prop_or_default]
    pub high_contrast: HighContrastProp,
    #[prop_or_default]
    pub radius: RadiusProp,
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
    pub as_child: Option<Callback<BadgeChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "span")]
pub struct BadgeChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub data_accent_color: String,
    pub data_radius: Option<String>,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn Badge(props: &BadgeProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.size,
            &props.variant,
            &props.color,
            &props.high_contrast,
            &props.radius,
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

    let child_props = BadgeChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-reset", "rt-Badge", class).to_string(),
        data_accent_color: props
            .color
            .0
            .map(|color| color.to_string())
            .unwrap_or_default(),
        data_radius: props.radius.0.map(|radius| radius.to_string()),
        id: props.id.clone(),
        style,
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
