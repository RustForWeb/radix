use radix_yew_primitive::compose_callbacks;
use radix_yew_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use radix_yew_use_previous::use_previous;
use radix_yew_use_size::use_size;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct SwitchContextValue {
    checked: bool,
    disabled: bool,
}

#[derive(PartialEq, Properties)]
pub struct SwitchProps {
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
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SwitchChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SwitchChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub r#type: String,
    pub role: String,
    pub aria_checked: String,
    pub aria_required: String,
    pub data_state: String,
    pub data_disabled: Option<String>,
    pub disabled: bool,
    pub value: String,
    pub onclick: Callback<MouseEvent>,
}

impl SwitchChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <button
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                type={self.r#type}
                role={self.role}
                aria-checked={self.aria_checked}
                aria-required={self.aria_required}
                data-state={self.data_state}
                data-disabled={self.data_disabled}
                disabled={self.disabled}
                value={self.value}
                onclick={self.onclick}
            >
                {children}
            </button>
        }
    }
}

#[function_component]
pub fn Switch(props: &SwitchProps) -> Html {
    let button_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), button_ref.clone()]);

    let is_form_control = use_state_eq(|| false);
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
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone(),
        r#type: "button".into(),
        role: "switch".into(),
        aria_checked: match checked {
            true => "true",
            false => "false",
        }
        .into(),
        aria_required: match props.required {
            true => "true",
            false => "false",
        }
        .into(),
        data_state: get_state(checked),
        data_disabled: props.disabled.then_some("".into()),
        disabled: props.disabled,
        value: props.value.clone(),
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
                    name={props.name.clone()}
                    control_ref={button_ref}
                    bubbles={true}
                    value={props.value.clone()}
                    checked={checked}
                    required={props.required}
                    disabled={props.disabled}
                />
            }
        </ContextProvider<SwitchContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct SwitchThumbProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub as_child: Option<Callback<SwitchThumbChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq)]
pub struct SwitchThumbChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub data_state: String,
    pub data_disabled: Option<String>,
}

impl SwitchThumbChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <span
                ref={self.node_ref}
                id={self.id}
                class={self.class}
                style={self.style}
                data-state={self.data_state}
                data-disabled={self.data_disabled}
            >
                {children}
            </span>
        }
    }
}

#[function_component]
pub fn SwitchThumb(props: &SwitchThumbProps) -> Html {
    let context = use_context::<SwitchContextValue>().expect("Switch context required.");

    let child_props = SwitchThumbChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: props.class.clone(),
        style: props.style.clone(),
        data_state: get_state(context.checked),
        data_disabled: context.disabled.then_some("".into()),
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
    pub checked: bool,
    #[prop_or(true)]
    pub bubbles: bool,
    #[prop_or_default]
    pub name: Option<String>,
    pub required: bool,
    pub disabled: bool,
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
            if let Some(input) = node_ref.cast::<web_sys::HtmlInputElement>() {
                if **prev_checked != *checked {
                    let init = web_sys::EventInit::new();
                    init.set_bubbles(*bubbles);

                    let event = web_sys::Event::new_with_event_init_dict("click", &init)
                        .expect("Click event should be instantiated.");

                    input.set_checked(*checked);

                    input
                        .dispatch_event(&event)
                        .expect("Click event should be dispatched.");
                }
            }
        },
    );

    html! {
        <input
            ref={node_ref}
            type="checkbox"
            aria-hidden="true"
            checked={props.checked}
            name={props.name.clone()}
            required={props.required}
            disabled={props.disabled}
            value={props.value.clone()}
            tabindex="-1"
            // We transform because the input is absolutely positioned, but we have
            // rendered it **after** the button. This pulls it back to sit on top
            // of the button.
            style={format!(
                "transform: translateX(-100%);{}{} position: absolute; pointer-events: none; opacity: 0; margin: 0px;",
                control_size.as_ref().map(|size| format!("{}px", size.width)).unwrap_or("".into()),
                control_size.as_ref().map(|size| format!("{}px", size.height)).unwrap_or("".into()),
            )}
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
