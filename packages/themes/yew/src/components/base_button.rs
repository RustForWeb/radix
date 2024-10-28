use yew::prelude::*;

use crate::{
    components::{
        base_button_props::{BaseButtonLoadingProp, BaseButtonSizeProp, BaseButtonVariantProp},
        flex::Flex,
        flex_props::{FlexAlign, FlexAs, FlexJustify},
        visually_hidden::VisuallyHidden,
    },
    helpers::{extract_props::extract_props, merge_classes::merge_classes, merge_styles::Style},
    props::{
        color_prop::AccentColorProp,
        high_contrast_prop::HighContrastProp,
        layout_props::Position,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        radius_prop::RadiusProp,
    },
};

#[derive(PartialEq, Properties)]
pub struct BaseButtonProps {
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

    // Attributes for `button`
    #[prop_or_default]
    pub autofocus: bool,
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
    pub as_child: Option<Callback<BaseButtonChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct BaseButtonChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: String,
    pub style: Style,
    pub autofocus: bool,
    pub command: Option<String>,
    pub commandfor: Option<String>,
    pub form: Option<String>,
    pub formaction: Option<String>,
    pub formenctype: Option<String>,
    pub formmethod: Option<String>,
    pub formnovalidate: bool,
    pub formtarget: Option<String>,
    pub name: Option<String>,
    pub popovertarget: Option<String>,
    pub popovertargetaction: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<String>,
    pub on_click: Callback<MouseEvent>,

    pub disabled: bool,
    pub data_disabled: Option<String>,
    pub data_accent_color: String,
    pub data_radius: Option<String>,
}

impl BaseButtonChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <button
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style.to_string()}
                autofocus={self.autofocus}
                command={self.command}
                commandfor={self.commandfor}
                form={self.form}
                formaction={self.formaction}
                formenctype={self.formenctype}
                formmethod={self.formmethod}
                formnovalidate={self.formnovalidate}
                formtarget={self.formtarget}
                name={self.name}
                popovertarget={self.popovertarget}
                popovertargetaction={self.popovertargetaction}
                type={self.r#type}
                value={self.value}
                onclick={self.on_click}

                disabled={self.disabled}
                data-disabled={self.data_disabled}
                data-accent-color={self.data_accent_color}
                data-radius={self.data_radius}
            >
                {children}
            </button>
        }
    }
}

#[function_component]
pub fn BaseButton(props: &BaseButtonProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.size,
            &props.variant,
            &props.color,
            &props.high_contrast,
            &props.radius,
            &props.loading,
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
    let disabled = props.disabled.unwrap_or(props.loading.0);

    let child_props = BaseButtonChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: merge_classes(&[&"rt-reset", &"rt-BaseButton", &class]),
        style,
        autofocus: props.autofocus,
        command: props.command.clone(),
        commandfor: props.commandfor.clone(),
        form: props.form.clone(),
        formaction: props.formaction.clone(),
        formenctype: props.formenctype.clone(),
        formmethod: props.formmethod.clone(),
        formnovalidate: props.formnovalidate,
        formtarget: props.formtarget.clone(),
        name: props.name.clone(),
        popovertarget: props.popovertarget.clone(),
        popovertargetaction: props.popovertargetaction.clone(),
        r#type: props.r#type.clone(),
        value: props.value.clone(),
        on_click: props.on_click.clone(),

        disabled,
        data_disabled: disabled.then_some("".into()),
        data_accent_color: props
            .color
            .0
            .map(|color| color.to_string())
            .unwrap_or("".to_string()),
        data_radius: props.radius.0.map(|radius| radius.to_string()),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(html! {
            if props.loading.0 {
                <>
                    // We need a wrapper to set `visibility: hidden` to hide the button content whilst we show the `Spinner`.
                    // The button is a flex container with a `gap`, so we use `display: contents` to ensure the correct flex layout.
                    //
                    // However, `display: contents` removes the content from the accessibility tree in some browsers,
                    // so we force remove it with `aria-hidden` and re-add it in the tree with `VisuallyHidden`.
                    <span style="display: contents; visibility: hidden;" aria-hidden="true">
                        {props.children.clone()}
                    </span>
                    <VisuallyHidden>{props.children.clone()}</VisuallyHidden>

                    <Flex r#as={FlexAs::Span} align={FlexAlign::Center} justify={FlexJustify::Center} position={Position::Absolute} inset=0>
                        // TODO
                        // <Spinner size={} />
                    </Flex>
                </>
            } else {
                {props.children.clone()}
            }
        })
    }
}
