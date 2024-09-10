use leptos::{
    ev::MouseEvent,
    html::{AnyElement, Input},
    *,
};
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_primitive::{compose_callbacks, Primitive};
use radix_leptos_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use radix_leptos_use_previous::use_previous;
use radix_leptos_use_size::use_size;

#[derive(Clone, Debug)]
struct SwitchContextValue {
    checked: Signal<bool>,
    disabled: Signal<bool>,
}

#[component]
pub fn Switch(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] default_checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let name = Signal::derive(move || name.get());
    let required = Signal::derive(move || required.get().unwrap_or(false));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let value = Signal::derive(move || value.get().unwrap_or("on".into()));

    let button_ref = NodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, button_ref]);

    let is_form_control = Signal::derive(move || {
        button_ref
            .get()
            .and_then(|button| button.closest("form").ok())
            .flatten()
            .is_some()
    });
    let (checked, set_checked) = use_controllable_state(UseControllableStateParams {
        prop: checked,
        on_change: on_checked_change.map(|on_checked_change| {
            Callback::new(move |value| {
                if let Some(value) = value {
                    on_checked_change.call(value);
                }
            })
        }),
        default_prop: default_checked,
    });
    let checked = Signal::derive(move || checked.get().unwrap_or(false));

    let context_value = SwitchContextValue { checked, disabled };

    let mut attrs = attrs.clone();
    attrs.extend([
        ("type", "button".into_attribute()),
        ("role", "switch".into_attribute()),
        (
            "aria-checked",
            (move || match checked.get() {
                true => "true",
                false => "false",
            })
            .into_attribute(),
        ),
        (
            "aria-required",
            (move || match required.get() {
                true => "true",
                false => "false",
            })
            .into_attribute(),
        ),
        (
            "data-state",
            (move || get_state(checked.get())).into_attribute(),
        ),
        (
            "data-disabled",
            (move || disabled.get().then_some("")).into_attribute(),
        ),
        (
            "disabled",
            (move || disabled.get().then_some("")).into_attribute(),
        ),
        ("value", value.into_attribute()),
    ]);

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=composed_refs
                attrs=attrs
                on:click=compose_callbacks(on_click, Some(Callback::new(move |event: MouseEvent| {
                    set_checked.call(Some(!checked.get()));

                    if is_form_control.get() {
                        // If switch is in a form, stop propagation from the button, so that we only propagate
                        // one click event (from the input). We propagate changes from an input so that native
                        // form validation works and form events reflect switch updates.
                        event.stop_propagation();
                    }
                })), None)
            >
                {children()}
            </Primitive>
            <Show when=move || is_form_control.get()>
                <BubbleInput
                    attr:name=name
                    control_ref=button_ref
                    bubbles=Signal::derive(|| true)
                    value=value
                    checked=checked
                    required=required
                    disabled=disabled
                />
            </Show>
        </Provider>
    }
}

#[component]
pub fn SwitchThumb(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SwitchContextValue>();

    let mut attrs = attrs.clone();
    attrs.extend([
        (
            "data-state",
            (move || get_state(context.checked.get())).into_attribute(),
        ),
        (
            "data-disabled",
            (move || context.disabled.get().then_some("")).into_attribute(),
        ),
    ]);

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            attrs=attrs
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

#[component]
fn BubbleInput(
    #[prop(into)] control_ref: NodeRef<AnyElement>,
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] bubbles: Signal<bool>,
    #[prop(into)] required: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] value: Signal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let node_ref: NodeRef<Input> = NodeRef::new();
    let prev_checked = use_previous(checked);
    let control_size = use_size(control_ref);

    // Bubble checked change to parents
    Effect::new(move |_| {
        if let Some(input) = node_ref.get() {
            if prev_checked.get() != checked.get() {
                let init = web_sys::EventInit::new();
                init.set_bubbles(bubbles.get());

                let event = web_sys::Event::new_with_event_init_dict("click", &init)
                    .expect("Click event should be instantiated.");

                input.set_checked(checked.get());

                input
                    .dispatch_event(&event)
                    .expect("Click event should be dispatched.");
            }
        }
    });

    view! {
        <input
            node_ref=node_ref
            type="checkbox"
            aria-hidden="true"
            checked=move || checked.get().then_some("")
            required=move || required.get().then_some("")
            disabled=move || disabled.get().then_some("")
            value=value
            tab-index="-1"
            // We transform because the input is absolutely positioned, but we have
            // rendered it **after** the button. This pulls it back to sit on top
            // of the button.
            style:transform="translateX(-100%)"
            style:width=move || control_size.get().map(|size| format!("{}px", size.width))
            style:height=move || control_size.get().map(|size| format!("{}px", size.height))
            style:position="absolute"
            style:pointer-events="none"
            style:opacity="0"
            style:margin="0px"
            {..attrs}
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
