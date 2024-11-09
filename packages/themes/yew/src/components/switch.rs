use radix_yew_switch::{Switch as SwitchPrimitive, SwitchThumb as SwitchThumbPrimitive};
use yew::prelude::*;

use crate::{
    components::switch_props::{SwitchSizeProp, SwitchVariantProp},
    helpers::{extract_props::extract_props, merge_styles::Style},
    props::{
        color_prop::ColorProp,
        high_contrast_prop::HighContrastProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        radius_prop::RadiusProp,
    },
};

#[derive(PartialEq, Properties)]
pub struct SwitchProps {
    #[prop_or_default]
    pub size: SwitchSizeProp,
    #[prop_or_default]
    pub variant: SwitchVariantProp,
    #[prop_or_default]
    pub color: ColorProp,
    #[prop_or_default]
    pub high_contrast: HighContrastProp,
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

    // Props from `SwitchPrimitive`
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub default_checked: Option<bool>,
    #[prop_or_default]
    pub on_checked_change: Callback<bool>,

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
    pub node_ref: NodeRef,
}

#[function_component]
pub fn Switch(props: &SwitchProps) -> Html {
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
        props.style.clone().into(),
    );

    html! {
        <SwitchPrimitive
            node_ref={props.node_ref.clone()}
            attributes={[
                ("data-accent-color", Some(props.color.0.map(|color| color.to_string()).unwrap_or_default())),
                ("data-radius", props.radius.0.map(|radius| radius.to_string()))
            ]}

            class={classes!("rt-reset", "rt-SwitchRoot", class).to_string()}
            id={props.id.clone()}
            style={style.to_string()}

            checked={props.checked}
            default_checked={props.default_checked}
            on_checked_change={props.on_checked_change.clone()}

            disabled={props.disabled}
            name={props.name.clone()}
            required={props.required}
            value={props.value.clone()}

            on_click={props.on_click.clone()}
        >
            <SwitchThumbPrimitive
                class={classes!(
                    "rt-SwitchThumb",
                    props.high_contrast.is_some_and(|high_contrast| high_contrast).then_some("rt-high-contrast"),
                ).to_string()}
            />
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
