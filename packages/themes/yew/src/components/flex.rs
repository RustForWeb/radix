use std::collections::HashMap;

use yew::prelude::*;

use crate::{
    components::flex_props::{
        FlexAlignProp, FlexAs, FlexAsProp, FlexDirectionProp, FlexDisplayProp, FlexJustifyProp,
        FlexWrapProp,
    },
    helpers::{extract_props::extract_props, merge_classes::merge_classes},
    props::{
        gap_props::{GapProp, GapXProp, GapYProp},
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
    },
};

#[derive(PartialEq, Properties)]
pub struct FlexProps {
    #[prop_or_default]
    pub r#as: FlexAsProp,
    #[prop_or_default]
    pub display: FlexDisplayProp,
    #[prop_or_default]
    pub direction: FlexDirectionProp,
    #[prop_or_default]
    pub align: FlexAlignProp,
    #[prop_or_default]
    pub justify: FlexJustifyProp,
    #[prop_or_default]
    pub wrap: FlexWrapProp,
    #[prop_or_default]
    pub gap: GapProp,
    #[prop_or_default]
    pub gap_x: GapXProp,
    #[prop_or_default]
    pub gap_y: GapYProp,
    // TODO: layout props
    #[prop_or_default]
    pub m: MProp,
    #[prop_or_default]
    pub mx: MxProp,
    #[prop_or_default]
    pub my: MyProp,
    #[prop_or_default]
    pub mr: MrProp,
    #[prop_or_default]
    pub mt: MtProp,
    #[prop_or_default]
    pub mb: MbProp,
    #[prop_or_default]
    pub ml: MlProp,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<HashMap<String, String>>,
    #[prop_or_default]
    pub as_child: Option<Callback<FlexChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct FlexChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: String,
    pub style: String,
    pub r#as: FlexAs,
}

impl FlexChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            if self.r#as == FlexAs::Div {
                <div
                    ref={self.node_ref}
                    id={self.id}
                    class={self.class}
                    style={self.style}
                >
                    {children}
                </div>
            } else {
                <span
                    ref={self.node_ref}
                    id={self.id}
                    class={self.class}
                    style={self.style}
                >
                    {children}
                </span>
            }
        }
    }
}

#[function_component]
pub fn Flex(props: &FlexProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.r#as,
            &props.display,
            &props.direction,
            &props.align,
            &props.justify,
            &props.wrap,
            &props.gap,
            &props.gap_x,
            &props.gap_y,
            &props.m,
            &props.mx,
            &props.my,
            &props.mt,
            &props.mr,
            &props.mb,
            &props.ml,
        ],
        props.class.clone(),
        props.style.clone(),
    );

    let child_props = FlexChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: merge_classes(&[&"rt-Flex", &class]),
        // TODO: abstract into Style class
        style: style
            .into_iter()
            .map(|(key, value)| format!("{key}: {value};"))
            .collect::<Vec<_>>()
            .join(" "),
        r#as: props.r#as.0,
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
