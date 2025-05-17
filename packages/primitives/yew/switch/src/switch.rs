use radix_yew_primitive::compose_callbacks;
use radix_yew_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use radix_yew_use_previous::use_previous;
use radix_yew_use_size::use_size;
use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent, struct_component};
use yew_style::Style;

#[derive(Clone, Debug, PartialEq)]
struct SwitchContextValue {
    checked: bool,
    disabled: bool,
}

#[derive(PartialEq, Properties)]
pub struct SwitchProps {
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
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<SwitchChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "button")]
pub struct SwitchChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub aria_checked: String,
    pub aria_required: String,
    pub class: Option<String>,
    pub data_disabled: Option<String>,
    pub data_state: String,
    pub id: Option<String>,
    pub role: String,
    pub style: Style,

    // Attributes from `button`
    pub disabled: bool,
    pub r#type: String,
    pub value: String,

    // Event handler attributes
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Switch(props: &SwitchProps) -> Html {
    let button_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), button_ref.clone()]);

    // We set this to true by default so that events bubble to forms without JS (SSR).
    let is_form_control = use_state_eq(|| true);
    use_effect_with(button_ref.clone(), {
        let is_form_control = is_form_control.clone();

        move |button_ref| {
            is_form_control.set(
                button_ref
                    .cast::<web_sys::Element>()
                    .and_then(|button| button.closest("form").ok())
                    .flatten()
                    .is_some(),
            );
        }
    });

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

    let context_value = use_memo((checked, props.disabled), |(checked, disabled)| {
        SwitchContextValue {
            checked: *checked,
            disabled: *disabled,
        }
    });

    let on_click = compose_callbacks(
        Some(props.on_click.clone()),
        Some(Callback::from({
            let is_form_control = is_form_control.clone();

            move |event: MouseEvent| {
                set_checked.emit(Some(!checked));

                if *is_form_control {
                    // If switch is in a form, stop propagation from the button, so that we only propagate
                    // one click event (from the input). We propagate changes from an input so that native
                    // form validation works and form events reflect switch updates.
                    event.stop_propagation();
                }
            }
        })),
        None,
    );

    let child_props = SwitchChildProps {
        node_ref: composed_refs,
        attributes: props.attributes.clone(),

        // Global attributes
        aria_checked: if checked { "true" } else { "false" }.to_owned(),
        aria_required: if props.required { "true" } else { "false" }.to_owned(),
        class: props.class.clone(),
        data_disabled: props.disabled.then_some("".to_owned()),
        data_state: get_state(checked),
        id: props.id.clone(),
        role: "switch".to_owned(),
        style: props.style.clone(),

        // Attributes from `button`
        disabled: props.disabled,
        r#type: "button".to_owned(),
        value: props.value.clone(),

        // Event handler attributes
        onclick: on_click.clone(),
    };

    html! {
        <ContextProvider<SwitchContextValue> context={(*context_value).clone()}>
            if let Some(as_child) = props.as_child.as_ref() {
                {as_child.emit(child_props)}
            } else {
                {child_props.render(props.children.clone())}
            }
            if *is_form_control {
                <BubbleInput
                    control_ref={button_ref}
                    bubbles={true}

                    style={[
                        // We transform because the input is absolutely positioned, but we have
                        // rendered it **after** the button. This pulls it back to sit on top
                        // of the button.
                        ("translate", "translateX(-100%)"),
                    ]}

                    checked={checked}
                    disabled={props.disabled}
                    name={props.name.clone()}
                    required={props.required}
                    value={props.value.clone()}
                />
            }
        </ContextProvider<SwitchContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct SwitchThumbProps {
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
    pub as_child: Option<Callback<SwitchThumbChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "span")]
pub struct SwitchThumbChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub data_disabled: Option<String>,
    pub data_state: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn SwitchThumb(props: &SwitchThumbProps) -> Html {
    let context = use_context::<SwitchContextValue>().expect("Switch context required.");

    let child_props = SwitchThumbChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: props.class.clone(),
        data_disabled: context.disabled.then_some("".to_owned()),
        data_state: get_state(context.checked),
        id: props.id.clone(),
        style: props.style.clone(),
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}

#[derive(PartialEq, Properties)]
struct BubbleInputProps {
    #[prop_or_default]
    pub control_ref: NodeRef,
    #[prop_or(true)]
    pub bubbles: bool,

    // Global attributes
    #[prop_or_default]
    pub style: Style,

    // Attributes from `input`
    pub checked: bool,
    pub disabled: bool,
    #[prop_or_default]
    pub name: Option<String>,
    pub required: bool,
    pub value: String,
}

#[function_component]
fn BubbleInput(props: &BubbleInputProps) -> Html {
    let node_ref = use_node_ref();
    let prev_checked = use_previous(props.checked);
    let control_size = use_size(props.control_ref.clone());

    // Bubble checked change to parents
    use_effect_with(
        (node_ref.clone(), prev_checked, props.checked, props.bubbles),
        |(node_ref, prev_checked, checked, bubbles)| {
            if let Some(input) = node_ref.cast::<web_sys::HtmlInputElement>()
                && **prev_checked != *checked
            {
                let init = web_sys::EventInit::new();
                init.set_bubbles(*bubbles);

                let event = web_sys::Event::new_with_event_init_dict("click", &init)
                    .expect("Click event should be instantiated.");

                input.set_checked(*checked);

                input
                    .dispatch_event(&event)
                    .expect("Click event should be dispatched.");
            }
        },
    );

    html! {
        <input
            ref={node_ref}

            aria-hidden="true"
            style={props.style.clone().with_defaults([
                ("width", control_size.as_ref().map(|size| format!("{}px", size.width))),
                ("height", control_size.as_ref().map(|size| format!("{}px", size.height))),
                ("position", Some("absolute".to_owned())),
                ("pointer-events", Some("none".to_owned())),
                ("opacity", Some("0".to_owned())),
                ("margin", Some("0px".to_owned())),
            ])}
            tabindex="-1"

            checked={props.checked}
            disabled={props.disabled}
            name={props.name.clone()}
            required={props.required}
            type="checkbox"
            value={props.value.clone()}
        />
    }
}

fn get_state(checked: bool) -> String {
    (match checked {
        true => "checked",
        false => "unchecked",
    })
    .into()
}
