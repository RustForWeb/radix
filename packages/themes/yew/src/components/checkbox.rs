pub use radix_yew_checkbox::CheckedState;
use radix_yew_checkbox::{
    Checkbox as CheckboxPrimitive, CheckboxIndicator as CheckboxIndicatorPrimitive,
    CheckboxIndicatorChildProps as CheckboxIndicatorPrimitiveChildProps,
};
use radix_yew_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use yew::prelude::*;
use yew_struct_component::Attributes;
use yew_style::Style;

use crate::{
    components::{
        checkbox_props::{CheckboxSizeProp, CheckboxVariantProp},
        icons::{ThickCheckIcon, ThickDividerHorizontalIcon},
    },
    helpers::extract_props::extract_props,
    props::{
        color_prop::AccentColorProp,
        high_contrast_prop::HighContrastProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
    },
};

#[derive(PartialEq, Properties)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub size: CheckboxSizeProp,
    #[prop_or_default]
    pub variant: CheckboxVariantProp,
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

    // Props from `CheckboxPrimitive`
    #[prop_or_default]
    pub checked: Option<CheckedState>,
    #[prop_or_default]
    pub default_checked: Option<CheckedState>,
    #[prop_or_default]
    pub on_checked_change: Callback<CheckedState>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `button`
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or("on".to_owned())]
    pub value: String,

    // Event handler attributes
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_key_down: Callback<KeyboardEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
}

#[function_component]
pub fn Checkbox(props: &CheckboxProps) -> Html {
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

    let on_change = use_callback(
        props.on_checked_change.clone(),
        |value: Option<CheckedState>, on_checked_change| {
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
    let handle_checked_change = use_callback(set_checked, |value, set_checked| {
        set_checked.emit(Some(value))
    });

    html! {
        <CheckboxPrimitive
            checked={checked}
            default_checked={props.default_checked}
            on_checked_change={handle_checked_change}

            class={classes!("rt-reset", "rt-BaseCheckboxRoot", "rt-CheckboxRoot", class).to_string()}
            id={props.id.clone()}
            style={style}

            disabled={props.disabled}
            name={props.name.clone()}
            required={props.required}
            value={props.value.clone()}

            on_click={props.on_click.clone()}
            on_key_down={props.on_key_down.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone().with_defaults([
                ("data-accent-color", props.color.0.map(|color| color.to_string()))
            ])}
        >
            <CheckboxIndicatorPrimitive
                class={"rt-BaseCheckboxIndicator rt-CheckboxIndicator"}

                as_child={Callback::from(move |CheckboxIndicatorPrimitiveChildProps {
                    node_ref,

                    class,
                    data_disabled,
                    data_state,
                    id,
                    style,

                    ..
                }| html! {
                    if checked == Some(CheckedState::Indeterminate) {
                        <ThickDividerHorizontalIcon
                            class={class}
                            data_disabled={data_disabled}
                            data_state={data_state}
                            id={id}
                            style={style}

                            node_ref={node_ref}
                        />
                    } else {
                        <ThickCheckIcon
                            class={class}
                            data_disabled={data_disabled}
                            data_state={data_state}
                            id={id}
                            style={style}

                            node_ref={node_ref}
                        />
                    }
                })}
            />
        </CheckboxPrimitive>
    }
}
