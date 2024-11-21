use yew::{prelude::*, virtual_dom::VNode};
use yew_struct_component::{struct_component, Attributes, StructComponent};
use yew_style::Style;

use crate::{
    components::skeleton_props::SkeletonLoadingProp,
    helpers::extract_props::extract_props,
    props::{
        height_props::{HeightProp, MaxHeightProp, MinHeightProp},
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        width_props::{MaxWidthProp, MinWidthProp, WidthProp},
    },
};

#[derive(PartialEq, Properties)]
pub struct SkeletonProps {
    #[prop_or_default]
    pub loading: SkeletonLoadingProp,
    #[prop_or_default]
    pub width: WidthProp,
    #[prop_or_default]
    pub min_width: MinWidthProp,
    #[prop_or_default]
    pub max_width: MaxWidthProp,
    #[prop_or_default]
    pub height: HeightProp,
    #[prop_or_default]
    pub min_height: MinHeightProp,
    #[prop_or_default]
    pub max_height: MaxHeightProp,
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
    pub as_child: Option<Callback<SkeletonChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "span")]
pub struct SkeletonChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub aria_hidden: Option<String>,
    pub class: Option<String>,
    pub data_inline_skeleton: Option<String>,
    pub id: Option<String>,
    pub inert: Option<String>,
    pub style: Style,
    pub tabindex: Option<String>,
}

#[function_component]
pub fn Skeleton(props: &SkeletonProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.loading,
            &props.width,
            &props.min_width,
            &props.max_width,
            &props.height,
            &props.min_height,
            &props.max_height,
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

    let child_props = if props.loading.0 {
        SkeletonChildProps {
            node_ref: props.node_ref.clone(),
            attributes: props.attributes.clone(),

            // Global attributes
            aria_hidden: Some("true".to_owned()),
            class: Some(classes!("rt-Skeleton", class).to_string()),
            data_inline_skeleton: matches!(&props.children, VNode::VList(list) if list.is_empty())
                .then_some("".to_owned()),
            id: props.id.clone(),
            inert: Some("".to_owned()),
            style,
            tabindex: Some("-1".to_owned()),
        }
    } else {
        // Workaround for `Slot`
        SkeletonChildProps {
            node_ref: props.node_ref.clone(),
            attributes: props.attributes.clone(),

            aria_hidden: None,
            class: None,
            data_inline_skeleton: None,
            inert: None,
            style: Style::default(),
            id: None,
            tabindex: None,
        }
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
