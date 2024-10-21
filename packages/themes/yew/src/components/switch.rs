use std::collections::HashMap;

use radix_yew_switch::{Switch as SwitchPrimitive, SwitchThumb as SwitchThumbPrimitive};
use yew::prelude::*;

use crate::{
    components::switch_props::{SwitchSize, SwitchVariant},
    helpers::{extract_props::extract_props, merge_classes::merge_classes},
    props::{
        color_prop::Color,
        high_contrast_prop::HighContrast,
        margin_props::{Mb, Ml, Mr, Mt, Mx, My, M},
        radius_prop::Radius,
    },
};

#[derive(PartialEq, Properties)]
pub struct SwitchProps {
    #[prop_or_default]
    pub size: SwitchSize,
    #[prop_or_default]
    pub variant: SwitchVariant,
    #[prop_or_default]
    pub color: Color,
    #[prop_or_default]
    pub high_contrast: HighContrast,
    #[prop_or_default]
    pub radius: Option<Radius>,
    #[prop_or_default]
    pub m: M,
    #[prop_or_default]
    pub mx: Mx,
    #[prop_or_default]
    pub mr: Mr,
    #[prop_or_default]
    pub mt: Mt,
    #[prop_or_default]
    pub my: My,
    #[prop_or_default]
    pub mb: Mb,
    #[prop_or_default]
    pub ml: Ml,

    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub default_checked: Option<bool>,
    #[prop_or_default]
    pub on_checked_change: Callback<bool>,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or("on".to_string())]
    pub value: String,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<HashMap<String, String>>,
}

#[function_component]
pub fn Switch(props: &SwitchProps) -> Html {
    // TODO: classNames(...)
    // TODO: data-*
    // TODO: other props

    let (class, style) = extract_props(
        &[
            &props.size,
            &props.variant,
            &props.color,
            &props.high_contrast,
            &props.radius,
            &props.m,
            &props.mx,
            &props.my,
            &props.mt,
            &props.mr,
            &props.mb,
            &props.ml,
            &props.radius,
        ],
        props.class.clone(),
        props.style.clone(),
    );

    html! {
        // TODO: data-accent-color, data-radius
        <SwitchPrimitive
            node_ref={props.node_ref.clone()}
            class={merge_classes(&[&"rt-reset", &"rt-SwitchRoot", &class])}
            style={style.into_iter().map(|(key, value)| format!("{key}: {value};")).collect::<Vec<_>>().join(" ")}
            name={props.name.clone()}
            checked={props.checked}
            default_checked={props.default_checked}
            on_checked_change={props.on_checked_change.clone()}
            required={props.required}
            disabled={props.disabled}
            value={props.value.clone()}
            on_click={props.on_click.clone()}
        >
            <SwitchThumbPrimitive class={merge_classes(&[&"rt-SwitchThumb", &("rt-high-contrast", &props.high_contrast)])} />
        </SwitchPrimitive>
    }
}

#[function_component]
fn Test() -> Html {
    use crate::props::{color_prop::AccentColor, radius_prop::Radius};

    html! {
        <Switch high_contrast=true color={AccentColor::Pink} radius={Radius::Small} m="10px" />
    }
}
