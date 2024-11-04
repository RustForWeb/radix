use yew::{prelude::*, virtual_dom::VNode};

use crate::{
    components::{
        flex::Flex,
        flex_props::{FlexAlign, FlexAs, FlexJustify},
        spinner_props::{SpinnerLoadingProp, SpinnerSizeProp},
    },
    helpers::{extract_props::extract_props, merge_classes::merge_classes, merge_styles::Style},
    props::{
        layout_props::Position,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
    },
};

#[derive(PartialEq, Properties)]
pub struct SpinnerProps {
    #[prop_or_default]
    pub size: SpinnerSizeProp,
    #[prop_or_default]
    pub loading: SpinnerLoadingProp,
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

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Spinner(props: &SpinnerProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.size,
            &props.loading,
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

    if !props.loading.0 {
        return props.children.clone();
    }

    let spinner = html! {
        <span
            ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={merge_classes(&[&"rt-Spinner", &class])}
            style={style.to_string()}
        >
            <span class="rt-SpinnerLeaf" />
            <span class="rt-SpinnerLeaf" />
            <span class="rt-SpinnerLeaf" />
            <span class="rt-SpinnerLeaf" />
            <span class="rt-SpinnerLeaf" />
            <span class="rt-SpinnerLeaf" />
            <span class="rt-SpinnerLeaf" />
            <span class="rt-SpinnerLeaf" />
        </span>
    };

    if matches!(&props.children, VNode::VList(list) if list.is_empty()) {
        spinner
    } else {
        html! {
            <Flex r#as={FlexAs::Span} position={Position::Relative} align={FlexAlign::Center} justify={FlexJustify::Center}>
                // `display: contents` removes the content from the accessibility tree in some browsers,
                // so we force remove it with `aria-hidden`.
                <span
                    aria-hidden=""
                    style="display: contents; visibility: hidden;"
                >
                    {props.children.clone()}
                </span>

                <Flex r#as={FlexAs::Span} align={FlexAlign::Center} justify={FlexJustify::Center} position={Position::Absolute} inset=0>
                    {spinner}
                </Flex>
            </Flex>
        }
    }
}
