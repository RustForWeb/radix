use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use radix_yew_presence::{Presence, PresenceChildProps};
use radix_yew_primitive::compose_callbacks;
use radix_yew_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use radix_yew_use_previous::use_previous;
use radix_yew_use_size::use_size;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};
use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent, struct_component};
use yew_style::Style;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CheckedState {
    False,
    True,
    Indeterminate,
}

impl Display for CheckedState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CheckedState::False => "false",
                CheckedState::True => "true",
                CheckedState::Indeterminate => "mixed",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
struct CheckboxContextValue {
    state: CheckedState,
    disabled: bool,
}

#[derive(PartialEq, Properties)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub checked: Option<CheckedState>,
    #[prop_or_default]
    pub default_checked: Option<CheckedState>,
    #[prop_or_default]
    pub on_checked_change: Callback<CheckedState>,

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
    pub on_key_down: Callback<KeyboardEvent>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<CheckboxChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "button")]
pub struct CheckboxChildProps {
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
    pub onkeydown: Callback<KeyboardEvent>,
}

#[function_component]
pub fn Checkbox(props: &CheckboxProps) -> Html {
    let button_ref = use_node_ref();
    let button = use_state_eq(|| None);
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), button_ref.clone()]);

    // We set this to true by default so that events bubble to forms without JS (SSR).
    let is_form_control = use_state_eq(|| true);
    use_effect_with(button_ref.clone(), {
        let button = button.clone();
        let is_form_control = is_form_control.clone();

        move |button_ref| {
            button.set(button_ref.cast::<web_sys::HtmlButtonElement>());
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
        |value: Option<CheckedState>, on_checked_change| {
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
    let checked = checked.unwrap_or(CheckedState::False);

    let initial_checked_state = use_mut_ref(|| checked);

    let handle_reset: Rc<Closure<dyn Fn(Event)>> = Rc::new(Closure::new({
        let set_checked = set_checked.clone();

        move |_| {
            set_checked.emit(Some(*initial_checked_state.borrow()));
        }
    }));
    use_effect_with(button, |button| {
        let mut cleanup: Option<Box<dyn Fn()>> = None;

        let form = button.as_ref().and_then(|button| button.form());
        if let Some(form) = form {
            form.add_event_listener_with_callback(
                "reset",
                (*handle_reset).as_ref().unchecked_ref(),
            )
            .expect("Reset event listener should be added.");

            cleanup = Some(Box::new(move || {
                form.remove_event_listener_with_callback(
                    "reset",
                    (*handle_reset).as_ref().unchecked_ref(),
                )
                .expect("Reset event listener should be removed.");
            }));
        }

        move || {
            if let Some(cleanup) = cleanup {
                cleanup();
            }
        }
    });

    let context_value = use_memo((checked, props.disabled), |(checked, disabled)| {
        CheckboxContextValue {
            state: *checked,
            disabled: *disabled,
        }
    });

    let onclick = compose_callbacks(
        Some(props.on_click.clone()),
        Some(Callback::from({
            let is_form_control = is_form_control.clone();

            move |event: MouseEvent| {
                set_checked.emit(Some(match checked {
                    CheckedState::False => CheckedState::True,
                    CheckedState::True => CheckedState::False,
                    CheckedState::Indeterminate => CheckedState::True,
                }));

                if *is_form_control {
                    // If checkbox is in a form, stop propagation from the button, so that we only propagate
                    // one click event (from the input). We propagate changes from an input so that native
                    // form validation works and form events reflect checkbox updates.
                    event.stop_propagation();
                }
            }
        })),
        None,
    );

    let onkeydown = compose_callbacks(
        Some(props.on_key_down.clone()),
        Some(Callback::from(|event: KeyboardEvent| {
            // According to WAI ARIA, checkboxes don't activate on enter keypress.
            if event.key() == "Enter" {
                event.prevent_default();
            }
        })),
        None,
    );

    let child_props = CheckboxChildProps {
        node_ref: composed_refs,
        attributes: props.attributes.clone(),

        // Global attributes
        aria_checked: checked.to_string(),
        aria_required: if props.required { "true" } else { "false" }.to_owned(),
        class: props.class.clone(),
        data_disabled: props.disabled.then_some("".to_owned()),
        data_state: get_state(checked),
        id: props.id.clone(),
        role: "checkbox".to_owned(),
        style: props.style.clone(),

        // Attributes from `button`
        disabled: props.disabled,
        r#type: "button".to_owned(),
        value: props.value.clone(),

        // Event handler attributes
        onclick,
        onkeydown,
    };

    html! {
        <ContextProvider<CheckboxContextValue> context={(*context_value).clone()}>
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
                        ("transform", "translateX(-100%)"),
                    ]}

                    checked={checked}
                    disabled={props.disabled}
                    name={props.name.clone()}
                    required={props.required}
                    value={props.value.clone()}
                />
            }
        </ContextProvider<CheckboxContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct CheckboxIndicatorProps {
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop_or_default]
    pub force_mount: Option<bool>,

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
    pub as_child: Option<Callback<CheckboxIndicatorChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "span")]
pub struct CheckboxIndicatorChildProps {
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
pub fn CheckboxIndicator(props: &CheckboxIndicatorProps) -> Html {
    let context = use_context::<CheckboxContextValue>().expect("Checkbox context required.");

    html! {
        <Presence
            present={props.force_mount.unwrap_or_default() || is_indeterminiate(context.state) || context.state == CheckedState::True}

            node_ref={props.node_ref.clone()}
            as_child={Callback::from({
                let class = props.class.clone();
                let id = props.id.clone();
                let style = props.style.clone();

                let attributes = props.attributes.clone();
                let as_child = props.as_child.clone();
                let children = props.children.clone();

                move |PresenceChildProps { node_ref }| {
                    let child_props = CheckboxIndicatorChildProps {
                        node_ref,
                        attributes: attributes.clone(),

                        // Global attributes
                        class: class.clone(),
                        data_disabled: context.disabled.then_some("".to_owned()),
                        data_state: get_state(context.state),
                        id: id.clone(),
                        style: style.clone().with_defaults([
                            ("pointer-events", "none")
                        ]),
                    };

                    if let Some(as_child) = as_child.as_ref() {
                        as_child.emit(child_props)
                    } else {
                        child_props.render(children.clone())
                    }
                }
            })}
        />
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
    pub checked: CheckedState,
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

                input.set_indeterminate(is_indeterminiate(*checked));
                input.set_checked(match checked {
                    CheckedState::False => false,
                    CheckedState::True => true,
                    CheckedState::Indeterminate => false,
                });

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

            checked={match props.checked {
                CheckedState::False => false,
                CheckedState::True => true,
                CheckedState::Indeterminate => false,
            }}
            disabled={props.disabled}
            name={props.name.clone()}
            required={props.required}
            type="checkbox"
            value={props.value.clone()}
        />
    }
}

fn is_indeterminiate(checked: CheckedState) -> bool {
    checked == CheckedState::Indeterminate
}

fn get_state(checked: CheckedState) -> String {
    (match checked {
        CheckedState::True => "checked",
        CheckedState::False => "unchecked",
        CheckedState::Indeterminate => "indeterminate",
    })
    .into()
}
