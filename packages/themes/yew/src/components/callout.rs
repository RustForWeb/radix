use html::IntoPropValue;
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};
use yew_style::Style;

use crate::{
    components::{
        callout_props::{CalloutSizeProp, CalloutVariantProp},
        text::Text,
        text_props::{TextAs, TextSizeProp},
    },
    helpers::extract_props::extract_props,
    props::{
        color_prop::{AccentColorProp, ColorProp},
        high_contrast_prop::HighContrastProp,
        leading_trim_prop::LeadingTrimProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        text_align_prop::TextAlignProp,
        text_wrap_prop::TextWrapProp,
        truncate_prop::TruncateProp,
        weight_prop::WeightProp,
    },
};

#[derive(Clone, PartialEq)]
struct CalloutContextValue {
    size: CalloutSizeProp,
}

#[derive(PartialEq, Properties)]
pub struct CalloutProps {
    #[prop_or_default]
    pub size: CalloutSizeProp,
    #[prop_or_default]
    pub variant: CalloutVariantProp,
    #[prop_or_default]
    pub color: AccentColorProp,
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
    pub role: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<CalloutChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct CalloutChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub data_accent_color: String,
    pub id: Option<String>,
    pub role: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn Callout(props: &CalloutProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.size,
            &props.variant,
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

    let child_props = CalloutChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-CalloutRoot", class).to_string(),
        data_accent_color: props
            .color
            .0
            .map(|color| color.to_string())
            .unwrap_or_default(),
        id: props.id.clone(),
        role: props.role.clone(),
        style,
    };

    let context_value = use_memo(props.size.clone(), |size| CalloutContextValue {
        size: size.clone(),
    });

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(html! {
            <ContextProvider<CalloutContextValue> context={(*context_value).clone()}>
                {props.children.clone()}
            </ContextProvider<CalloutContextValue>>
        })
    }
}

#[derive(PartialEq, Properties)]
pub struct CalloutIconProps {
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
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct CalloutIconChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn CalloutIcon(props: &CalloutIconProps) -> Html {
    let child_props = CalloutIconChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-CalloutIcon", &props.class).to_string(),
        id: props.id.clone(),
        style: props.style.clone(),
    };

    child_props.render(props.children.clone())
}

#[derive(PartialEq, Properties)]
pub struct CalloutTextProps {
    // Props from `Text`
    #[prop_or_default]
    pub size: TextSizeProp,
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
    pub children: Html,
}

#[function_component]
pub fn CalloutText(props: &CalloutTextProps) -> Html {
    let context = use_context::<CalloutContextValue>().expect("Callout context required.");

    html! {
        <Text
            r#as={TextAs::P}
            size={if props.size.0.is_some() { props.size.clone() } else { context.size.into_prop_value() }}
            weight={props.weight.clone()}
            align={props.align.clone()}
            trim={props.trim.clone()}
            truncate={props.truncate.clone()}
            wrap={props.wrap.clone()}
            color={props.color.clone()}
            high_contrast={props.high_contrast.clone()}
            m={props.m.clone()}
            mx={props.mx.clone()}
            my={props.my.clone()}
            mt={props.mt.clone()}
            mr={props.mr.clone()}
            mb={props.mb.clone()}
            ml={props.ml.clone()}

            class={classes!("rt-CalloutText", &props.class).to_string()}
            id={props.id.clone()}
            style={props.style.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
        >
            {props.children.clone()}
        </Text>
    }
}
