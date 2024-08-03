use leptos::{ev::MouseEvent, html::AnyElement, *};
use radix_leptos_primitive::{compose_callbacks, Primitive};
use radix_leptos_use_controllable_state::{use_controllable_state, UseControllableStateParams};

#[component]
pub fn Toggle(
    /// The controlled state of the toggle.
    #[prop(into, optional)]
    pressed: MaybeProp<bool>,
    /// The state of the toggle when initially rendered. Use `default_pressed` if you do not need to control the state of the toggle. Defaults to `false`.
    #[prop(into, optional)]
    default_pressed: MaybeProp<bool>,
    /// The callback that fires when the state of the toggle changes.
    #[prop(into, optional)]
    on_pressed_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let (pressed, set_pressed) = use_controllable_state(UseControllableStateParams {
        prop: pressed,
        on_change: on_pressed_change.map(|on_pressed_change| {
            Callback::new(move |value| {
                if let Some(value) = value {
                    on_pressed_change.call(value);
                }
            })
        }),
        default_prop: default_pressed,
    });
    let pressed = Signal::derive(move || pressed.get().unwrap_or(false));

    let mut attrs = attrs.clone();
    attrs.extend([
        ("aria-pressed", pressed.into_attribute()),
        (
            "data-state",
            (move || match pressed.get() {
                true => "on",
                false => "off",
            })
            .into_attribute(),
        ),
        (
            "data-disabled",
            (move || disabled.get().then_some("")).into_attribute(),
        ),
        (
            "disabled",
            (move || disabled.get().then_some("")).into_attribute(),
        ),
    ]);

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attrs=attrs
            on:click=compose_callbacks(on_click, Some(Callback::new(move |_| {
                if !disabled.get() {
                    set_pressed.call(Some(!pressed.get()));
                }
            })), None)
        >
            {children()}
        </Primitive>
    }
}
