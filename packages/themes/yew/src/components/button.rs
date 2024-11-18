use yew::prelude::*;
use yew_struct_component::Attributes;
use yew_style::Style;

use crate::{
    components::{
        base_button::{BaseButton, BaseButtonChildProps},
        button_props::{ButtonLoadingProp, ButtonSizeProp, ButtonVariantProp},
    },
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
    pub size: ButtonSizeProp,
    #[prop_or_default]
    pub variant: ButtonVariantProp,
    #[prop_or_default]
    pub color: AccentColorProp,
    #[prop_or_default]
    pub high_contrast: HighContrastProp,
    #[prop_or_default]
    pub radius: RadiusProp,
    #[prop_or_default]
    pub loading: ButtonLoadingProp,
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
    pub autofocus: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `button`
    #[prop_or_default]
    pub command: Option<String>,
    #[prop_or_default]
    pub commandfor: Option<String>,
    #[prop_or_default]
    pub disabled: Option<bool>,
    #[prop_or_default]
    pub form: Option<String>,
    #[prop_or_default]
    pub formaction: Option<String>,
    #[prop_or_default]
    pub formenctype: Option<String>,
    #[prop_or_default]
    pub formmethod: Option<String>,
    #[prop_or_default]
    pub formnovalidate: bool,
    #[prop_or_default]
    pub formtarget: Option<String>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub popovertarget: Option<String>,
    #[prop_or_default]
    pub popovertargetaction: Option<String>,
    #[prop_or_default]
    pub r#type: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,

    // Event handler attributes
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_pointer_down: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_leave: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_move: Callback<PointerEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
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

            autofocus={props.autofocus}
            class={classes!("rt-Button", &props.class).to_string()}
            id={props.id.clone()}
            style={props.style.clone()}

            command={props.command.clone()}
            commandfor={props.commandfor.clone()}
            disabled={props.disabled}
            form={props.form.clone()}
            formaction={props.formaction.clone()}
            formenctype={props.formenctype.clone()}
            formmethod={props.formmethod.clone()}
            formnovalidate={props.formnovalidate}
            formtarget={props.formtarget.clone()}
            name={props.name.clone()}
            popovertarget={props.popovertarget.clone()}
            popovertargetaction={props.popovertargetaction.clone()}
            r#type={props.r#type.clone()}
            value={props.value.clone()}

            on_blur={props.on_blur.clone()}
            on_click={props.on_click.clone()}
            on_focus={props.on_focus.clone()}
            on_pointer_down={props.on_pointer_down.clone()}
            on_pointer_leave={props.on_pointer_leave.clone()}
            on_pointer_move={props.on_pointer_move.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
        >
            {props.children.clone()}
        </BaseButton>
    }
}
