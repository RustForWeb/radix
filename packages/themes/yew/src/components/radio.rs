use radix_yew_primitive::compose_callbacks;
use radix_yew_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};
use yew_style::Style;

use crate::{
    components::radio_props::{RadioSizeProp, RadioVariantProp},
    helpers::extract_props::extract_props,
    props::{
        color_prop::ColorProp,
        high_contrast_prop::HighContrastProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
    },
};

#[derive(PartialEq, Properties)]
pub struct RadioProps {
    #[prop_or_default]
    pub size: RadioSizeProp,
    #[prop_or_default]
    pub variant: RadioVariantProp,
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
    pub checked: Option<bool>,
    #[prop_or_default]
    pub default_checked: Option<bool>,
    #[prop_or_default]
    pub on_checked_change: Callback<bool>,
    #[prop_or_default]
    pub on_value_change: Callback<String>,

    // Global attributes
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `input`
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub form: Option<String>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub required: bool,
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
#[struct_component(tag = "input")]
pub struct RadioChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub autofocus: bool,
    pub class: String,
    pub data_accent_color: Option<String>,
    pub id: Option<String>,
    pub style: Style,

    // Attributes from `input`
    pub checked: bool,
    pub disabled: bool,
    pub form: Option<String>,
    pub name: Option<String>,
    pub required: bool,
    pub r#type: String,
    pub value: Option<String>,

    // Event handler attributes
    pub onblur: Callback<FocusEvent>,
    pub onchange: Callback<Event>,
    pub onfocus: Callback<FocusEvent>,
    pub oninput: Callback<InputEvent>,
}

#[function_component]
pub fn Radio(props: &RadioProps) -> Html {
    let radio_ref = use_node_ref();
    let composed_ref = use_composed_ref(&[radio_ref.clone(), props.node_ref.clone()]);

    let on_change = use_callback(
        props.on_checked_change.clone(),
        |value: Option<bool>, on_checked_change| {
            if let Some(value) = value {
                on_checked_change.emit(value);
            }
        },
    );
    let (checked, set_checked) = use_controllable_state(UseControllableStateParams {
        prop: props.checked,
        on_change: Some(on_change),
        default_prop: props.default_checked,
    });
    let checked = checked.unwrap_or(false);

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

    let onchange = compose_callbacks(
        Some(props.on_change.clone()),
        Some(Callback::from({
            let on_value_change = props.on_value_change.clone();

            move |_| {
                // Yew messes up `current_target`, see https://yew.rs/docs/concepts/html/events#event-delegation.
                //
                // event.current_target().map(|current_target| current_target.unchecked_into::<web_sys::HtmlInputElement>())
                let radio = radio_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .expect("Node ref should be HtmlInputElement.");

                set_checked.emit(Some(radio.checked()));
                on_value_change.emit(radio.value());
            }
        })),
        None,
    );

    let child_props = RadioChildProps {
        node_ref: composed_ref,
        attributes: props.attributes.clone(),

        // Global attributes
        autofocus: props.autofocus,
        class: classes!("rt-reset", "rt-BaseRadioRoot", "rt-RadioRoot", class).to_string(),
        data_accent_color: props.color.0.map(|color| color.to_string()),
        id: props.id.clone(),
        style,

        // Attributes from `input`
        checked,
        disabled: props.disabled,
        form: props.form.clone(),
        name: props.name.clone(),
        required: props.required,
        r#type: "radio".to_owned(),
        value: props.value.clone(),

        // Event handler attributes
        onblur: props.on_blur.clone(),
        onchange,
        onfocus: props.on_focus.clone(),
        oninput: props.on_input.clone(),
    };

    child_props.render(props.children.clone())
}
