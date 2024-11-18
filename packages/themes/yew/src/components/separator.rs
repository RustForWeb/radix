// use radix_yew_separator::Separator as SeparatorPrimitive;
use yew::prelude::*;
use yew_style::Style;

use crate::{
    components::separator_props::{
        SeparatorDecorativeProp, SeparatorOrientationProp, SeparatorSizeProp,
    },
    helpers::extract_props::extract_props,
    props::{
        color_prop::{AccentColor, AccentColorProp},
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
    },
};

#[derive(PartialEq, Properties)]
pub struct SeparatorProps {
    #[prop_or_default]
    pub orientation: SeparatorOrientationProp,
    #[prop_or_default]
    pub size: SeparatorSizeProp,
    #[prop_or(AccentColorProp(Some(AccentColor::Gray)))]
    pub color: AccentColorProp,
    #[prop_or_default]
    pub decorative: SeparatorDecorativeProp,
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
    pub children: Html,
}

#[function_component]
pub fn Separator(props: &SeparatorProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.orientation,
            &props.size,
            &props.color,
            &props.decorative,
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

    html! {
        <span
            ref={props.node_ref.clone()}

            class={classes!("rt-Separator", class).to_string()}
            data-accent-color={props.color.0.unwrap_or(AccentColor::Gray).to_string()}
            id={props.id.clone()}
            role={(!props.decorative.0).then_some("separator")}
            style={style}
        >
            {props.children.clone()}
        </span>
    }

    // Apparently, this component is not actually based on the primitive.
    //
    // <SeparatorPrimitive
    //     orientation={props.orientation.0}
    //     decorative={props.decorative.0}
    //
    //     // data-accent-color
    //     class={classes!("rt-Separator", class).to_string()}
    //     id={props.id.clone()}
    //     style={style}
    //
    //     node_ref={props.node_ref.clone()}
    // >
    //     {props.children.clone()}
    // </SeparatorPrimitive>
}
