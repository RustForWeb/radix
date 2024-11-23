use radix_yew_direction::Direction;
use radix_yew_select::{
    Select as SelectPrimitive, SelectContent as SelectContentPrimitive,
    SelectGroup as SelectGroupPrimitive, SelectIcon as SelectIconPrimitive, SelectIconChildProps,
    SelectItem as SelectItemPrimitive, SelectItemIndicator as SelectItemIndicatorPrimitive,
    SelectItemText as SelectItemTextPrimitive, SelectLabel as SelectLabelPrimitive,
    SelectPortal as SelectPortalPrimitive, SelectSeparator as SelectSeparatorPrimitive,
    SelectTrigger as SelectTriggerPrimitive,
    SelectTriggerChildProps as SelectTriggerPrimitiveChildProps,
    SelectValue as SelectValuePrimitive, SelectViewport as SelectViewportPrimitive,
    SelectViewportChildProps,
};
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};
use yew_style::Style;

use crate::{
    components::{
        icons::{ChevronDownIcon, ThickCheckIcon},
        select_props::{SelectContentVariantProp, SelectSizeProp, SelectTriggerVariantProp},
        theme::{use_theme_context, Theme, ThemeChildProps},
    },
    helpers::extract_props::extract_props,
    props::{
        color_prop::ColorProp,
        high_contrast_prop::HighContrastProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        radius_prop::RadiusProp,
    },
};

#[derive(Clone, PartialEq)]
pub struct SelectContextValue {
    size: SelectSizeProp,
}

#[derive(PartialEq, Properties)]
pub struct SelectProps {
    #[prop_or_default]
    pub size: SelectSizeProp,

    // Props from `SelectPrimitive`
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub default_value: Option<String>,
    #[prop_or_default]
    pub on_value_change: Callback<String>,
    #[prop_or_default]
    pub open: Option<bool>,
    #[prop_or_default]
    pub default_open: Option<bool>,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,

    // Global attributes
    #[prop_or_default]
    pub dir: Option<Direction>,

    // Attributes from `select`
    #[prop_or_default]
    pub autocomplete: Option<String>,
    #[prop_or_default]
    pub disabled: Option<bool>,
    #[prop_or_default]
    pub form: Option<String>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub required: Option<bool>,

    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Select(props: &SelectProps) -> Html {
    let context_value = use_memo(props.size.clone(), |size| SelectContextValue {
        size: size.clone(),
    });

    html! {
        <SelectPrimitive
            value={props.value.clone()}
            default_value={props.default_value.clone()}
            on_value_change={props.on_value_change.clone()}
            open={props.open}
            default_open={props.default_open}
            on_open_change={props.on_open_change.clone()}

            dir={props.dir}

            autocomplete={props.autocomplete.clone()}
            disabled={props.disabled}
            form={props.form.clone()}
            name={props.name.clone()}
            required={props.required}
        >
            <ContextProvider<SelectContextValue> context={(*context_value).clone()}>
                {props.children.clone()}
            </ContextProvider<SelectContextValue>>
        </SelectPrimitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectTriggerProps {
    #[prop_or_default]
    pub variant: SelectTriggerVariantProp,
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

    // Props from `SelectValuePrimitive`
    #[prop_or("".to_owned())]
    pub placeholder: String,

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
#[struct_component(tag = "button")]
pub struct SelectTriggerChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub aria_controls: String,
    pub aria_expanded: String,
    pub aria_required: Option<String>,
    pub aria_autocomplete: String,
    pub data_accent_color: Option<String>,
    pub data_disabled: Option<String>,
    pub data_placeholder: Option<String>,
    pub data_radius: Option<String>,
    pub data_state: String,
    pub dir: String,
    pub class: String,
    pub id: Option<String>,
    pub role: String,
    pub style: Style,

    // Attributes from `button`
    pub disabled: bool,
    pub r#type: String,

    // Event handler attributes
    pub onclick: Callback<MouseEvent>,
    pub onkeydown: Callback<KeyboardEvent>,
    pub onpointerdown: Callback<PointerEvent>,
}

#[function_component]
pub fn SelectTrigger(props: &SelectTriggerProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");

    let (class, style) = extract_props(
        &[
            &context.size,
            &props.variant,
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
        <SelectTriggerPrimitive
            class={class.to_string()}
            id={props.id.clone()}
            style={style}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
            as_child={Callback::from({
                let color = props.color.0;
                let radius = props.radius.0;
                let placeholder = props.placeholder.clone();
                let children = props.children.clone();

                move |SelectTriggerPrimitiveChildProps {
                    node_ref,
                    attributes,

                    aria_autocomplete,
                    aria_controls,
                    aria_expanded,
                    aria_required,
                    class,
                    data_disabled,
                    data_placeholder,
                    data_state,
                    dir,
                    id,
                    role,
                    style,

                    disabled,
                    r#type,

                    onclick,
                    onpointerdown,
                    onkeydown,
                }| {
                    let child_props = SelectTriggerChildProps {
                        node_ref,
                        attributes,

                        // Global attributes
                        aria_autocomplete,
                        aria_controls,
                        aria_expanded,
                        aria_required,
                        class: classes!("rt-reset", "rt-SelectTrigger", class).to_string(),
                        data_accent_color: color.map(|color| color.to_string()),
                        data_disabled,
                        data_placeholder,
                        data_radius: radius.map(|radius| radius.to_string()),
                        data_state,
                        dir,
                        id,
                        role,
                        style,

                        // Attributes from `button`
                        disabled,
                        r#type,

                        // Event handler attributes
                        onclick,
                        onpointerdown,
                        onkeydown,
                    };

                    child_props.render(html! {
                        <>
                            <span class="rt-SelectTriggerInner">
                                <SelectValuePrimitive placeholder={placeholder.clone()}>
                                    {children.clone()}
                                </SelectValuePrimitive>
                            </span>
                            <SelectIconPrimitive
                                as_child={Callback::from(|SelectIconChildProps {..}| html! {
                                    <ChevronDownIcon class="rt-SelectIcon" />
                                })}
                            />
                        </>
                    })
                }
            })}
        />
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectContentProps {
    #[prop_or_default]
    pub variant: SelectContentVariantProp,
    #[prop_or_default]
    pub color: ColorProp,
    #[prop_or_default]
    pub high_contrast: HighContrastProp,

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
pub fn SelectContent(props: &SelectContentProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");

    let (class, style) = extract_props(
        &[
            &context.size,
            &props.variant,
            &props.color,
            &props.high_contrast,
        ],
        props.class.clone(),
        props.style.clone().into(),
    );

    let theme_context = use_theme_context();
    let resolved_color = props.color.0.unwrap_or(theme_context.accent_color);

    html! {
        // TODO: portal container prop
        <SelectPortalPrimitive>
            <Theme
                class={class.to_string()}
                id={props.id.clone()}
                style={style}

                node_ref={props.node_ref.clone()}
                attributes={props.attributes.clone()}
                as_child={Callback::from({
                    let children = props.children.clone();

                    move |ThemeChildProps {
                        node_ref,
                        attributes,

                        class,
                        data_accent_color: _data_accent_color,
                        data_gray_color,
                        data_has_background,
                        data_is_root_theme,
                        data_panel_background,
                        data_radius,
                        data_scaling,
                        id,
                        style,
                    }| html! {
                        <SelectContentPrimitive
                            // TODO
                            // side_offset={4}

                            // TODO: pass props

                            // TODO: popper class
                            class={classes!("rt-SelectContent", class).to_string()}
                            id={id}
                            style={style}

                            node_ref={node_ref}
                            attributes={attributes.with_defaults([
                                ("data-accent-color", resolved_color.to_string()),
                                ("data-gray-color", data_gray_color),
                                ("data-has-background", data_has_background),
                                ("data-is-root-theme", data_is_root_theme),
                                ("data-panel-background", data_panel_background),
                                ("data-radius", data_radius),
                                ("data-scaling", data_scaling),
                            ])}
                        >
                            // TODO: ScrollAreaPrimitive
                            <SelectViewportPrimitive
                                class="rt-SelectviewPort"
                                as_child={Callback::from({
                                    let children = children.clone();

                                    move |SelectViewportChildProps {
                                        // node_ref,
                                        // id,
                                        // class,
                                        // style,
                                        // data_radix_select_viewport,
                                        // role
                                        ..
                                    }| html! {
                                        // TODO: ScrollAreaViewportPrimitive
                                        {children.clone()}
                                    }
                                })}
                            >
                            </SelectViewportPrimitive>
                            // TODO: ScrollAreaScrollbarPrimitive, ScrollAreaThumbPrimitive
                        </SelectContentPrimitive>
                    }
                })}
            />
        </SelectPortalPrimitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectItemProps {
    // Props from `SelectItemPrimitive`
    pub value: String,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub text_value: String,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Event handler attributes
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_key_down: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub on_pointer_down: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_leave: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_move: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_up: Callback<PointerEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectItem(props: &SelectItemProps) -> Html {
    html! {
        <SelectItemPrimitive
            value={props.value.clone()}
            disabled={props.disabled}
            text_value={props.text_value.clone()}

            id={props.id.clone()}
            class={classes!("rt-SelectItem", &props.class).to_string()}
            style={props.style.clone()}

            on_blur={props.on_blur.clone()}
            on_focus={props.on_focus.clone()}
            on_click={props.on_click.clone()}
            on_key_down={props.on_key_down.clone()}
            on_pointer_down={props.on_pointer_down.clone()}
            on_pointer_leave={props.on_pointer_leave.clone()}
            on_pointer_move={props.on_pointer_move.clone()}
            on_pointer_up={props.on_pointer_up.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
        >
            <SelectItemIndicatorPrimitive class="rt-SelectItemIndicator">
                <ThickCheckIcon class="rt-SelectItemIndicatorIcon" />
            </SelectItemIndicatorPrimitive>
            <SelectItemTextPrimitive>{props.children.clone()}</SelectItemTextPrimitive>
        </SelectItemPrimitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectGroupProps {
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
pub fn SelectGroup(props: &SelectGroupProps) -> Html {
    html! {
        <SelectGroupPrimitive
            class={classes!("rt-SelectGroup", &props.class).to_string()}
            id={props.id.clone()}
            style={props.style.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
        >
            {props.children.clone()}
        </SelectGroupPrimitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectLabelProps {
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
pub fn SelectLabel(props: &SelectLabelProps) -> Html {
    html! {
        <SelectLabelPrimitive
            class={classes!("rt-SelectLabel", &props.class).to_string()}
            id={props.id.clone()}
            style={props.style.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
        >
            {props.children.clone()}
        </SelectLabelPrimitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectSeparatorProps {
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
pub fn SelectSeparator(props: &SelectSeparatorProps) -> Html {
    html! {
        <SelectSeparatorPrimitive
            class={classes!("rt-SelectSeparator", &props.class).to_string()}
            id={props.id.clone()}
            style={props.style.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
        >
            {props.children.clone()}
        </SelectSeparatorPrimitive>
    }
}
