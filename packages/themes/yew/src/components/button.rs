use yew::prelude::*;

use crate::{
    components::{
        base_button::{BaseButton, BaseButtonChildProps},
        base_button_props::{BaseButtonLoadingProp, BaseButtonSizeProp, BaseButtonVariantProp},
    },
    helpers::{merge_classes::merge_classes, merge_styles::Style},
    props::{
        color_prop::AccentColorProp,
        high_contrast_prop::HighContrastProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        radius_prop::RadiusProp,
    },
};

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub size: BaseButtonSizeProp,
    #[prop_or_default]
    pub variant: BaseButtonVariantProp,
    #[prop_or_default]
    pub color: AccentColorProp,
    #[prop_or_default]
    pub high_contrast: HighContrastProp,
    #[prop_or_default]
    pub radius: RadiusProp,
    #[prop_or_default]
    pub loading: BaseButtonLoadingProp,
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
    pub disabled: Option<bool>,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub as_child: Option<Callback<ButtonChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

pub type ButtonChildProps = BaseButtonChildProps;

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    html! {
        <BaseButton
            size={props.size.clone()}
            variant={props.variant.clone()}
            color={props.color.clone()}
            high_contrast={props.high_contrast.clone()}
            radius={props.radius.clone()}
            loading={props.loading.clone()}
            m={props.m.clone()}
            mx={props.mx.clone()}
            my={props.my.clone()}
            mt={props.mt.clone()}
            mr={props.mr.clone()}
            mb={props.mb.clone()}
            ml={props.ml.clone()}

            disabled={props.disabled}
            on_click={props.on_click.clone()}

            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={merge_classes(&[&"rt-Button", &props.class])}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </BaseButton>
    }
}
