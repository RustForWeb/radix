use std::fmt::{Display, Formatter};

use leptos::{html::AnyElement, *};

const DEFAULT_MAX: f64 = 100.0;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProgressState {
    Indeterminate,
    Complete,
    Loading,
}

impl Display for ProgressState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProgressState::Indeterminate => "indeterminate",
                ProgressState::Complete => "complete",
                ProgressState::Loading => "loading",
            }
        )
    }
}

impl IntoAttribute for ProgressState {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.to_string().into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

#[derive(Clone, Debug)]
struct ProgressContextValue {
    value: Signal<Option<f64>>,
    max: Signal<f64>,
}

#[component]
pub fn Progress(
    #[prop(into, optional)] value: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] get_value_label: Option<Box<dyn Fn(f64, f64) -> String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let get_value_label = get_value_label.unwrap_or(Box::new(default_get_value_label));
    let max = Signal::derive(move || {
        max.get()
            .and_then(|max| match max == 0.0 {
                true => None,
                false => Some(max),
            })
            .unwrap_or(DEFAULT_MAX)
    });
    let value = Signal::derive(move || value.get().map(|value| value.min(max.get()).max(0.0)));

    let value_label =
        Signal::derive(move || value.get().map(|value| get_value_label(value, max.get())));

    let context_value = ProgressContextValue { value, max };

    let mut attrs = attrs.clone();
    attrs.extend([
        ("aria-valuemax", max.into_attribute()),
        ("aria-valuemin", "0".into_attribute()),
        ("aria-valuenow", value.into_attribute()),
        ("aria-valuetext", value_label.into_attribute()),
        ("role", "progressbar".into_attribute()),
        (
            "data-state",
            (move || get_progress_state(value.get(), max.get())).into_attribute(),
        ),
        ("data-value", value.into_attribute()),
        ("data-max", max.into_attribute()),
    ]);

    view! {
        <Provider value=context_value>
             <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attrs=attrs
            >
                {children()}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn ProgressIndicator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ProgressContextValue>();

    let mut attrs = attrs.clone();
    attrs.extend([
        (
            "data-state",
            (move || get_progress_state(context.value.get(), context.max.get())).into_attribute(),
        ),
        ("data-value", context.value.into_attribute()),
        ("data-max", context.max.into_attribute()),
    ]);
    let attrs = StoredValue::new(attrs);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attrs=attrs.get_value()
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

fn default_get_value_label(value: f64, max: f64) -> String {
    format!("{}%", (value / max).round() * 100.0)
}

fn get_progress_state(value: Option<f64>, max_value: f64) -> ProgressState {
    match value {
        Some(value) => match value == max_value {
            true => ProgressState::Complete,
            false => ProgressState::Loading,
        },
        None => ProgressState::Indeterminate,
    }
}
