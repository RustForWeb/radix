use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};
use yew_style::Style;

use crate::{
    components::text_area_props::{TextAreaResizeProp, TextAreaSizeProp, TextAreaVariantProp},
    helpers::extract_props::extract_props,
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

    // Global attributes
    #[prop_or_default]
    pub autocapitalize: Option<String>,
    #[prop_or_default]
    pub autocorrect: Option<String>,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub spellcheck: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `textarea`
    #[prop_or_default]
    pub autocomplete: Option<String>,
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
    pub value: Option<String>,

    // Event handler attributes
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
    pub attributes: Attributes,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "textarea")]
pub struct TextAreaChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub autocapitalize: Option<String>,
    pub autocorrect: Option<String>,
    pub autofocus: bool,
    pub class: String,
    pub id: Option<String>,
    pub spellcheck: Option<String>,

    // Attributes from `textarea`
    pub autocomplete: Option<String>,
    pub cols: Option<String>,
    pub dirname: Option<String>,
    pub disabled: bool,
    pub form: Option<String>,
    pub maxlength: Option<String>,
    pub minlength: Option<String>,
    pub name: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub readonly: bool,
    pub required: bool,
    pub rows: Option<String>,
    pub value: Option<String>,

    // Event handler attributes
    pub onblur: Callback<FocusEvent>,
    pub onchange: Callback<Event>,
    pub onfocus: Callback<FocusEvent>,
    pub oninput: Callback<InputEvent>,
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

    let child_props = TextAreaChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        autocapitalize: props.autocapitalize.clone(),
        autocorrect: props.autocorrect.clone(),
        autofocus: props.autofocus,
        class: "rt-reset rt-TextAreaInput".to_owned(),
        id: props.id.clone(),
        spellcheck: props.spellcheck.clone(),

        // Attributes from `textarea`
        autocomplete: props.autocomplete.clone(),
        cols: props.cols.clone(),
        dirname: props.dirname.clone(),
        disabled: props.disabled,
        form: props.form.clone(),
        maxlength: props.maxlength.clone(),
        minlength: props.minlength.clone(),
        name: props.name.clone(),
        pattern: props.pattern.clone(),
        placeholder: props.placeholder.clone(),
        readonly: props.readonly,
        required: props.required,
        rows: props.rows.clone(),
        value: props.value.clone(),

        // Event handler attributes
        onblur: props.on_blur.clone(),
        onchange: props.on_change.clone(),
        onfocus: props.on_focus.clone(),
        oninput: props.on_input.clone(),
    };

    html! {
        <div
            class={classes!("rt-TextAreaRoot", class).to_string()}
            style={style}
            data-accent-color={props.color.0.map(|color| color.to_string())}
            data-radius={props.radius.0.map(|radius| radius.to_string())}
        >
            {child_props.render(props.children.clone())}
        </div>
    }
}
