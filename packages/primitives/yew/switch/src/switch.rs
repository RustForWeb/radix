use radix_yew_primitive::{compose_callbacks, Primitive};
use radix_yew_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use radix_yew_use_previous::use_previous;
use radix_yew_use_size::use_size;
use yew::prelude::*;
use yew_attrs::{attrs, Attrs};

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
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
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

    let attrs = use_memo(
        (
            props.attrs.clone(),
            props.required,
            props.disabled,
            props.value.clone(),
            checked,
        ),
        |(attrs, required, disabled, value, checked)| {
            attrs
                .clone()
                .merge(attrs! {
                    type="button"
                    role="switch"
                    aria-checked={match checked {
                        true => "true",
                        false => "false",
                    }}
                    aria-required={match required {
                        true => "true",
                        false => "false",
                    }}
                    data-state={get_state(*checked)}
                    data-disabled={disabled.then_some("")}
                    disabled={*disabled}
                    value={value.clone()}
                    onclick={on_click}
                })
                .expect("Attributes should be merged.")
        },
    );

    html! {
        <ContextProvider<SwitchContextValue> context={(*context_value).clone()}>
            <Primitive
                element="button"
                as_child={props.as_child}
                node_ref={composed_refs}
                attrs={(*attrs).clone()}
            >
                {props.children.clone()}
            </Primitive>
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
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SwitchThumb(props: &SwitchThumbProps) -> Html {
    let context = use_context::<SwitchContextValue>().expect("Switch context required.");

    let attrs = use_memo((props.attrs.clone(), context), |(attrs, context)| {
        attrs
            .clone()
            .merge(attrs! {
                data-state={get_state(context.checked)}
                data-disabled={context.disabled.then_some("")}
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <Primitive
            element="span"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
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
    #[prop_or_default]
    pub attrs: Attrs,
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

    let attrs = use_memo(
        (
            props.attrs.clone(),
            props.checked,
            props.name.clone(),
            props.required,
            props.disabled,
            props.value.clone(),
            control_size,
        ),
        |(attrs, checked, name, required, disabled, value, control_size)| {
            attrs
            .clone()
            .merge(attrs! {
                type="checkbox"
                aria-hidden="true"
                checked={*checked}
                name={(*name).clone()}
                required={*required}
                disabled={*disabled}
                value={value.clone()}
                tab-index="-1"
                // We transform because the input is absolutely positioned, but we have
                // rendered it **after** the button. This pulls it back to sit on top
                // of the button.
                style={format!(
                    "transform: translateX(-100%);{}{} position: absolute; pointer-events: none; opacity: 0; margin: 0px;",
                    control_size.as_ref().map(|size| format!("{}px", size.width)).unwrap_or("".into()),
                    control_size.as_ref().map(|size| format!("{}px", size.height)).unwrap_or("".into()),
                )}
            })
            .expect("Attributes should be merged.")
        },
    );

    (*attrs)
        .clone()
        .new_vtag("input", node_ref, Default::default(), Default::default())
        .into()
}

fn get_state(checked: bool) -> String {
    (match checked {
        true => "checked",
        false => "unchecked",
    })
    .into()
}
