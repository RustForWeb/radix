use yew::prelude::*;

use crate::{
    components::text_area_props::{TextAreaResizeProp, TextAreaSizeProp, TextAreaVariantProp},
    helpers::{extract_props::extract_props, merge_classes::merge_classes, merge_styles::Style},
    props::{
        color_prop::ColorProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        radius_prop::RadiusProp,
    },
};

#[derive(PartialEq, Properties)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub size: TextAreaSizeProp,
    #[prop_or_default]
    pub variant: TextAreaVariantProp,
    #[prop_or_default]
    pub resize: TextAreaResizeProp,
    #[prop_or_default]
    pub color: ColorProp,
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

    // Attributes from `textarea`
    #[prop_or_default]
    pub autocapitalize: Option<String>,
    #[prop_or_default]
    pub autocomplete: Option<String>,
    #[prop_or_default]
    pub autocorrect: Option<String>,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub cols: Option<String>,
    #[prop_or_default]
    pub dirname: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub form: Option<String>,
    #[prop_or_default]
    pub maxlength: Option<String>,
    #[prop_or_default]
    pub minlength: Option<String>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub pattern: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub rows: Option<String>,
    #[prop_or_default]
    pub spellcheck: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_change: Callback<Event>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_input: Callback<InputEvent>,

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
pub fn TextArea(props: &TextAreaProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.size,
            &props.variant,
            &props.resize,
            &props.color,
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

    html! {
        <div
            class={merge_classes(&[&"rt-TextAreaRoot", &class])}
            style={style.to_string()}
            data-accent-color={props.color.0.map(|color| color.to_string())}
            data-radius={props.radius.0.map(|radius| radius.to_string())}
        >
            <textarea
                ref={props.node_ref.clone()}
                id={props.id.clone()}
                class="rt-reset rt-TextAreaInput"
                autocapitalize={props.autocapitalize.clone()}
                autocomplete={props.autocomplete.clone()}
                autocorrect={props.autocorrect.clone()}
                autofocus={props.autofocus}
                cols={props.cols.clone()}
                dirname={props.dirname.clone()}
                disabled={props.disabled}
                form={props.form.clone()}
                maxlength={props.maxlength.clone()}
                minlength={props.minlength.clone()}
                name={props.name.clone()}
                pattern={props.pattern.clone()}
                placeholder={props.placeholder.clone()}
                readonly={props.readonly}
                required={props.required}
                rows={props.rows.clone()}
                spellcheck={props.spellcheck.clone()}
                value={props.value.clone()}
                onblur={props.on_blur.clone()}
                onchange={props.on_change.clone()}
                onfocus={props.on_focus.clone()}
                oninput={props.on_input.clone()}
            />
        </div>
    }
}
