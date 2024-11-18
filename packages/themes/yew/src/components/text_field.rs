use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    window,
};
use yew::prelude::*;
use yew_struct_component::{struct_component, Attributes, StructComponent};
use yew_style::Style;

use crate::{
    components::text_field_props::{
        TextFieldSizeProp, TextFieldSlotSideProp, TextFieldVariantProp,
    },
    helpers::extract_props::extract_props,
    props::{
        color_prop::ColorProp,
        gap_props::GapProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        padding_props::{PlProp, PrProp, PxProp},
        radius_prop::RadiusProp,
    },
};

#[derive(PartialEq, Properties)]
pub struct TextFieldProps {
    #[prop_or_default]
    pub size: TextFieldSizeProp,
    #[prop_or_default]
    pub variant: TextFieldVariantProp,
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
    pub style: Style,
    #[prop_or_default]
    pub spellcheck: Option<String>,

    // Attributes from `input`
    #[prop_or_default]
    pub accept: Option<String>,
    #[prop_or_default]
    pub alt: Option<String>,
    #[prop_or_default]
    pub autocomplete: Option<String>,
    #[prop_or_default]
    pub capture: Option<String>,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub dirname: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
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
    pub height: Option<String>,
    #[prop_or_default]
    pub list: Option<String>,
    #[prop_or_default]
    pub max: Option<String>,
    #[prop_or_default]
    pub maxlength: Option<String>,
    #[prop_or_default]
    pub min: Option<String>,
    #[prop_or_default]
    pub minlength: Option<String>,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub pattern: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub popovertarget: Option<String>,
    #[prop_or_default]
    pub popovertargetaction: Option<String>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub src: Option<String>,
    #[prop_or_default]
    pub step: Option<String>,
    #[prop_or_default]
    pub r#type: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub width: Option<String>,

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
pub struct TextFieldChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub autocapitalize: Option<String>,
    pub autocorrect: Option<String>,
    pub autofocus: bool,
    pub class: String,
    pub id: Option<String>,
    pub spellcheck: String,

    // Attributes from `input`
    pub accept: Option<String>,
    pub alt: Option<String>,
    pub autocomplete: Option<String>,
    pub capture: Option<String>,
    pub checked: bool,
    pub dirname: Option<String>,
    pub disabled: bool,
    pub form: Option<String>,
    pub formaction: Option<String>,
    pub formenctype: Option<String>,
    pub formmethod: Option<String>,
    pub formnovalidate: bool,
    pub formtarget: Option<String>,
    pub height: Option<String>,
    pub list: Option<String>,
    pub max: Option<String>,
    pub maxlength: Option<String>,
    pub min: Option<String>,
    pub minlength: Option<String>,
    pub multiple: bool,
    pub name: Option<String>,
    pub pattern: Option<String>,
    pub placeholder: Option<String>,
    pub popovertarget: Option<String>,
    pub popovertargetaction: Option<String>,
    pub readonly: bool,
    pub required: bool,
    pub src: Option<String>,
    pub step: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<String>,
    pub width: Option<String>,

    // Event handler attributes
    pub onblur: Callback<FocusEvent>,
    pub onchange: Callback<Event>,
    pub onfocus: Callback<FocusEvent>,
    pub oninput: Callback<InputEvent>,
}

#[function_component]
pub fn TextField(props: &TextFieldProps) -> Html {
    let input_ref = use_node_ref();
    let composed_ref = use_composed_ref(&[input_ref.clone(), props.node_ref.clone()]);

    let (class, style) = extract_props(
        &[
            &props.size,
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

    let last_animation_frame = use_mut_ref(|| None::<(i32, Closure<dyn Fn()>)>);

    let onpointerdown = use_callback((), {
        let last_animation_frame = last_animation_frame.clone();

        move |event: PointerEvent, _| {
            let target = event.target_unchecked_into::<web_sys::HtmlElement>();
            if target.closest("input, button, a").ok().flatten().is_some() {
                return;
            }

            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                // Same selector as in the CSS to find the right slot.
                let is_right_slot = target.closest("\
                        .rt-TextFieldSlot[data-side='right'],\
                        .rt-TextFieldSlot:not([data-side='right']) ~ .rt-TextFieldSlot:not([data-side='left'])\
                    ").ok().flatten().is_some();

                let cursor_position = is_right_slot
                    .then(|| input.value().len() as u32)
                    .unwrap_or(0);

                let closure = Closure::new(move || {
                    // Only some input types support this, browsers will throw an error if not supported.
                    // See: https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setSelectionRange.
                    _ = input.set_selection_range(cursor_position, cursor_position);

                    input.focus().expect("Element should be focused.");
                });

                let animation_handle = window()
                    .expect("Window should exist")
                    .request_animation_frame(closure.as_ref().unchecked_ref())
                    .expect("Animation frame should be requested.");

                *last_animation_frame.borrow_mut() = Some((animation_handle, closure));
            }
        }
    });

    use_effect(move || {
        move || {
            if let Some((animation_handle, _)) = last_animation_frame.take() {
                window()
                    .expect("Window should exist.")
                    .cancel_animation_frame(animation_handle)
                    .expect("Animation frame should be canceled.");
            }
        }
    });

    let child_props = TextFieldChildProps {
        node_ref: composed_ref,
        attributes: props.attributes.clone(),

        // Global attributes
        autocapitalize: props.autocapitalize.clone(),
        autocorrect: props.autocorrect.clone(),
        autofocus: props.autofocus,
        class: "rt-reset rt-TextFieldInput".to_owned(),
        id: props.id.clone(),
        spellcheck: props.spellcheck.clone().unwrap_or("false".to_owned()),

        // Attributes from `input`
        accept: props.accept.clone(),
        alt: props.alt.clone(),
        autocomplete: props.autocomplete.clone(),
        capture: props.capture.clone(),
        checked: props.checked,
        dirname: props.dirname.clone(),
        disabled: props.disabled,
        form: props.form.clone(),
        formaction: props.formaction.clone(),
        formenctype: props.formenctype.clone(),
        formmethod: props.formmethod.clone(),
        formnovalidate: props.formnovalidate,
        formtarget: props.formtarget.clone(),
        height: props.height.clone(),
        list: props.list.clone(),
        max: props.max.clone(),
        maxlength: props.maxlength.clone(),
        min: props.min.clone(),
        minlength: props.minlength.clone(),
        multiple: props.multiple,
        name: props.name.clone(),
        pattern: props.pattern.clone(),
        placeholder: props.placeholder.clone(),
        popovertarget: props.popovertarget.clone(),
        popovertargetaction: props.popovertargetaction.clone(),
        readonly: props.readonly,
        required: props.required,
        src: props.src.clone(),
        step: props.step.clone(),
        r#type: props.r#type.clone(),
        value: props.value.clone(),
        width: props.width.clone(),

        // Event handler attributes
        onblur: props.on_blur.clone(),
        onchange: props.on_change.clone(),
        onfocus: props.on_focus.clone(),
        oninput: props.on_input.clone(),
    };

    html! {
        <div
            class={classes!("rt-TextFieldRoot", class).to_string()}
            style={style}
            data-accent-color={props.color.0.map(|color| color.to_string())}
            data-radius={props.radius.0.map(|radius| radius.to_string())}
            onpointerdown={onpointerdown}
        >
            {child_props.render(props.children.clone())}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct TextFieldSlotProps {
    #[prop_or_default]
    pub side: TextFieldSlotSideProp,
    #[prop_or_default]
    pub color: ColorProp,
    #[prop_or_default]
    pub gap: GapProp,
    #[prop_or_default]
    pub px: PxProp,
    #[prop_or_default]
    pub pr: PrProp,
    #[prop_or_default]
    pub pl: PlProp,

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
    pub children: Html,
}

#[function_component]
pub fn TextFieldSlot(props: &TextFieldSlotProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.side,
            &props.color,
            &props.gap,
            &props.px,
            &props.pr,
            &props.pl,
        ],
        props.class.clone(),
        props.style.clone().into(),
    );

    html! {
        <div
            ref={props.node_ref.clone()}

            data-accent-color={props.color.0.map(|color| color.to_string())}
            data-side={props.side.0.map(|side| side.to_string())}
            class={classes!("rt-TextFieldSlot", class).to_string()}
            id={props.id.clone()}
            style={style}
        >
            {props.children.clone()}
        </div>
    }
}
