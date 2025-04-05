use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent, struct_component};
use yew_style::Style;

use crate::{
    components::{
        text::{Text, TextChildProps},
        text_props::TextSizeProp,
    },
    props::{
        color_prop::ColorProp,
        high_contrast_prop::HighContrastProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        text_wrap_prop::TextWrapProp,
        truncate_prop::TruncateProp,
        weight_prop::WeightProp,
    },
};

#[derive(PartialEq, Properties)]
pub struct BlockquoteProps {
    #[prop_or_default]
    pub size: TextSizeProp,
    #[prop_or_default]
    pub weight: WeightProp,
    #[prop_or_default]
    pub color: ColorProp,
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
    pub as_child: Option<Callback<BlockquoteChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "blockquote")]
pub struct BlockquoteChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub data_accent_color: Option<String>,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn Blockquote(props: &BlockquoteProps) -> Html {
    html! {
        <Text
            size={props.size.clone()}
            weight={props.weight.clone()}
            color={props.color.clone()}
            high_contrast={props.high_contrast.clone()}
            truncate={props.truncate.clone()}
            wrap={props.wrap.clone()}
            m={props.m.clone()}
            mx={props.mx.clone()}
            my={props.my.clone()}
            mt={props.mt.clone()}
            mr={props.mr.clone()}
            mb={props.mb.clone()}
            ml={props.ml.clone()}

            class={classes!("rt-Blockquote", &props.class).to_string()}
            id={props.id.clone()}
            style={props.style.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
            as_child={Callback::from({
                let as_child = props.as_child.clone();
                let children = props.children.clone();

                move |TextChildProps { node_ref, attributes, class, data_accent_color, id, style, .. }| {
                    let child_props = BlockquoteChildProps {
                        node_ref,
                        attributes,

                        class,
                        data_accent_color,
                        id,
                        style,
                    };

                    if let Some(as_child) = as_child.as_ref() {
                        as_child.emit(child_props)
                    } else {
                        child_props.render(children.clone())
                    }
                }
            })}
        />
    }
}
