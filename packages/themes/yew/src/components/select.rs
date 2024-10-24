use radix_yew_direction::Direction;
use radix_yew_select::{
    Select as SelectPrimitive, SelectContent as SelectContentPrimitive,
    SelectGroup as SelectGroupPrimitive, SelectIcon as SelectIconPrimitive, SelectIconChildProps,
    SelectItem as SelectItemPrimitive, SelectItemIndicator as SelectItemIndicatorPrimitive,
    SelectItemText as SelectItemTextPrimitive, SelectLabel as SelectLabelPrimitive,
    SelectPortal as SelectPortalPrimitive, SelectSeparator as SelectSeparatorPrimitive,
    SelectTrigger as SelectTriggerPrimitive, SelectTriggerChildProps,
    SelectValue as SelectValuePrimitive, SelectViewport as SelectViewportPrimitive,
    SelectViewportChildProps,
};
use yew::prelude::*;

use crate::{
    components::{
        icons::{ChevronDownIcon, ThickCheckIcon},
        select_props::{SelectContentVariantProp, SelectSizeProp, SelectTriggerVariantProp},
        theme::Theme,
    },
    helpers::{extract_props::extract_props, merge_classes::merge_classes, merge_styles::Style},
    props::{
        color_prop::ColorProp,
        high_contrast_prop::HighContrastProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        radius_prop::RadiusProp,
    },
    ThemeChildProps,
};

#[derive(Clone, PartialEq)]
pub struct SelectContextValue {
    size: SelectSizeProp,
}

#[derive(PartialEq, Properties)]
pub struct SelectProps {
    #[prop_or_default]
    pub size: SelectSizeProp,

    // Props from SelectPrimitive
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
    #[prop_or_default]
    pub dir: Option<Direction>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub autocomplete: Option<String>,
    #[prop_or_default]
    pub disabled: Option<bool>,
    #[prop_or_default]
    pub required: Option<bool>,
    #[prop_or_default]
    pub form: Option<String>,

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
            name={props.name.clone()}
            autocomplete={props.autocomplete.clone()}
            disabled={props.disabled}
            required={props.required}
            form={props.form.clone()}
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

    // Props from SelectValuePrimitive
    #[prop_or("".to_string())]
    pub placeholder: String,

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
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={class}
            style={style.to_string()}
            as_child={Callback::from({
                let color = props.color.0;
                let radius = props.radius.0;
                let placeholder = props.placeholder.clone();
                let children = props.children.clone();

                move |SelectTriggerChildProps {
                    node_ref,
                    id,
                    class,
                    style,
                    r#type,
                    role,
                    aria_controls,
                    aria_expanded,
                    aria_required,
                    aria_autocomplete,
                    dir,
                    data_state,
                    disabled,
                    data_disabled,
                    data_placeholder,
                    onclick,
                    onpointerdown,
                    onkeydown,
                }| html! {
                    <button
                        data-accent-color={color.map(|color| color.to_string())}
                        data-radius={radius.map(|radius| radius.to_string())}

                        ref={node_ref}
                        id={id}
                        class={merge_classes(&[&"rt-reset", &"rt-SelectTrigger", &class])}
                        style={style}
                        type={r#type}
                        role={role}
                        aria-controls={aria_controls}
                        aria-expanded={aria_expanded}
                        aria-required={aria_required}
                        aria-autocomplete={aria_autocomplete}
                        dir={dir}
                        data-state={data_state}
                        disabled={disabled}
                        data-disabled={data_disabled}
                        data-placeholder={data_placeholder}
                        onclick={onclick}
                        onpointerdown={onpointerdown}
                        onkeydown={onkeydown}
                    >
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
                    </button>
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

    html! {
        // TODO: portal container prop
        <SelectPortalPrimitive>
            <Theme
                node_ref={props.node_ref.clone()}
                id={props.id.clone()}
                class={class}
                style={style}
                as_child={Callback::from({
                    let children = props.children.clone();

                    move |ThemeChildProps {
                        node_ref,
                        id,
                        class,
                        style,
                        // data_is_root_theme,
                        // data_accent_color,
                        // data_gray_color,
                        // data_has_background,
                        // data_panel_background,
                        // data_radius,
                        // data_scaling
                        ..
                    }| html! {
                        <SelectContentPrimitive
                            // TODO
                            // data-accent-color={resolved_color}
                            // side_offset={4}

                            // TODO: pass props

                            node_ref={node_ref}
                            id={id}
                            style={style.to_string()}
                            // TODO: popper class
                            class={merge_classes(&[&"rt-SelectContent", &class])}
                            // TODO: pass attributes
                            // data-is-root-theme={data_is_root_theme}
                            // data-accent-color={data_accent_color}
                            // data-gray-color={data_gray_color}
                            // data-has-background={data_has_background}
                            // data-panel-background={data_panel_background}
                            // data-radius={data_radius}
                            // data-scaling={data_scaling}
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
    // Props from SelectItemPrimitive
    pub value: String,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub text_value: String,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_pointer_up: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_down: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_move: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_pointer_leave: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_key_down: Callback<KeyboardEvent>,

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
pub fn SelectItem(props: &SelectItemProps) -> Html {
    html! {
        <SelectItemPrimitive
            value={props.value.clone()}
            disabled={props.disabled}
            text_value={props.text_value.clone()}
            on_focus={props.on_focus.clone()}
            on_blur={props.on_blur.clone()}
            on_focus={props.on_focus.clone()}
            on_click={props.on_click.clone()}
            on_pointer_up={props.on_pointer_up.clone()}
            on_pointer_down={props.on_pointer_down.clone()}
            on_pointer_move={props.on_pointer_move.clone()}
            on_pointer_leave={props.on_pointer_leave.clone()}
            on_key_down={props.on_key_down.clone()}

            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={merge_classes(&[&"rt-SelectItem", &props.class])}
            style={props.style.to_string()}
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
pub fn SelectGroup(props: &SelectGroupProps) -> Html {
    html! {
        <SelectGroupPrimitive
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={merge_classes(&[&"rt-SelectGroup", &props.class])}
            style={props.style.to_string()}
        >
            {props.children.clone()}
        </SelectGroupPrimitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectLabelProps {
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
pub fn SelectLabel(props: &SelectLabelProps) -> Html {
    html! {
        <SelectLabelPrimitive
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={merge_classes(&[&"rt-SelectLabel", &props.class])}
            style={props.style.to_string()}
        >
            {props.children.clone()}
        </SelectLabelPrimitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectSeparatorProps {
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
pub fn SelectSeparator(props: &SelectSeparatorProps) -> Html {
    html! {
        <SelectSeparatorPrimitive
            node_ref={props.node_ref.clone()}
            id={props.id.clone()}
            class={merge_classes(&[&"rt-SelectSeparator", &props.class])}
            style={props.style.to_string()}
        >
            {props.children.clone()}
        </SelectSeparatorPrimitive>
    }
}
