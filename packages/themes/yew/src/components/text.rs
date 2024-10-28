use yew::prelude::*;

use crate::{
    components::text_props::{TextAs, TextAsProp, TextSizeProp},
    helpers::{extract_props::extract_props, merge_classes::merge_classes, merge_styles::Style},
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
pub struct TextProps {
    #[prop_or_default]
    pub r#as: TextAsProp,
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

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub as_child: Option<Callback<TextChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct TextChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: String,
    pub style: Style,
    pub r#as: TextAs,
    pub data_accent_color: Option<String>,
}

impl TextChildProps {
    pub fn render(self, children: Html) -> Html {
        match self.r#as {
            TextAs::Span => html! {
                <span
                    ref={self.node_ref}
                    id={self.id}
                    class={self.class}
                    style={self.style.to_string()}

                    data-accent-color={self.data_accent_color}
                >
                    {children}
                </span>
            },
            TextAs::Div => html! {
                <div
                    ref={self.node_ref}
                    id={self.id}
                    class={self.class}
                    style={self.style.to_string()}

                    data-accent-color={self.data_accent_color}
                >
                    {children}
                </div>
            },
            TextAs::Label => html! {
                <label
                    ref={self.node_ref}
                    id={self.id}
                    class={self.class}
                    style={self.style.to_string()}

                    data-accent-color={self.data_accent_color}
                >
                    {children}
                </label>
            },
            TextAs::P => html! {
                <p
                    ref={self.node_ref}
                    id={self.id}
                    class={self.class}
                    style={self.style.to_string()}

                    data-accent-color={self.data_accent_color}
                >
                    {children}
                </p>
            },
        }
    }
}

#[function_component]
pub fn Text(props: &TextProps) -> Html {
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

    let child_props = TextChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: merge_classes(&[&"rt-Text", &class]),
        style,
        r#as: props.r#as.0,
        data_accent_color: props.color.0.map(|color| color.to_string()),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
