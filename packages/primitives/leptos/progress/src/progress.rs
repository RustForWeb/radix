use leptos::context::Provider;
use leptos::{html, logging};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_primitive::Primitive;
use radix_leptos_context::create_context;

/* -------------------------------------------------------------------------------------------------
 * Progress Types & Constants
 * -----------------------------------------------------------------------------------------------*/

const DEFAULT_MAX: f64 = 100.0;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProgressState {
    Indeterminate,
    Complete,
    Loading,
}

impl std::fmt::Display for ProgressState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/* -------------------------------------------------------------------------------------------------
 * Progress Context
 * -----------------------------------------------------------------------------------------------*/

const PROGRESS_NAME: &'static str = "Progress";

#[derive(Clone, Debug)]
pub struct ProgressContextValue {
    /// Current progress value, or None for indeterminate.
    pub value: Signal<Option<f64>>,
    /// Maximum progress value.
    pub max: Signal<f64>,
}

create_context!(
    context_type: ProgressContextValue,
    provider: ProgressProvider,
    hook: use_progress_context,
    root: PROGRESS_NAME // for debug labels
);

/* -------------------------------------------------------------------------------------------------
 * Progress (Root)
 * -----------------------------------------------------------------------------------------------*/

#[component]
#[allow(non_snake_case)]
pub fn Progress(
    /// The current progress value.
    #[prop(into, optional)]
    value: MaybeProp<f64>,
    /// The maximum allowed progress value (defaults to 100).
    #[prop(into, optional, default=100.0.into())]
    max: MaybeProp<f64>,
    /// A callback `(f64, f64) -> String`. Defaults to a "percentage" label if not provided.
    #[prop(into, optional)]
    get_value_label: Option<Callback<(f64, f64), String>>,
    /// If `true`, renders the `<div>` as a child (no extra wrapper).
    #[prop(into, optional)]
    as_child: MaybeProp<bool>,
    /// A reference to the `<div>` node, if needed.
    #[prop(optional)]
    node_ref: AnyNodeRef,
    /// Child elements, usually `ProgressIndicator`.
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    // Provide a default callback if none is given.
    let get_value_label = get_value_label.unwrap_or_else(|| {
        Callback::new(|(value, max): (f64, f64)| {
            let pct = ((value / max) * f64::from(100.0)).round();
            format!("{}%", pct)
        })
    });

    // Derive signals for max and clamped value.
    let max_signal = Signal::derive({
        let max = max.clone();
        move || max.get().filter(|m| *m > 0.0).unwrap_or(DEFAULT_MAX)
    });

    let value_signal = Signal::derive({
        let value = value.clone();
        move || value.get().map(|v| v.clamp(0.0, max_signal.get()))
    });

    // Derive signals for data/aria attributes.
    let progress_state = Signal::derive({
        let val = value_signal.clone();
        let mx = max_signal.clone();
        move || get_progress_state(val.get(), mx.get())
    });

    let value_label = Signal::derive({
        let val = value_signal.clone();
        let mx = max_signal.clone();
        let label_cb = get_value_label; // Callback is Copy
        move || val.get().map(|v| label_cb.run((v, mx.get())))
    });

    // Add a reactive effect for warning checks
    #[debug_assertions]
    Effect::new(move |_| {
        let current_max = max_signal.get();
        if current_max <= 0.0 {
            logging::warn!(
                "Invalid prop `max` of value `{}` supplied to `Progress`. Defaulting to `{}`.",
                current_max,
                DEFAULT_MAX
            );
        }

        if let Some(v) = value_signal.get() {
            if v < 0.0 || v > current_max {
                logging::warn!(
                    "Invalid prop `value` of `{}` supplied to `Progress` (must be between 0 and {}). \
                     Defaulting to `None` (indeterminate).",
                    v,
                    current_max
                );
            }
        }
    });

    // Provide context to children.
    let ctx_value = ProgressContextValue {
        value: value_signal,
        max: max_signal,
    };

    view! {
        <ProgressProvider value=ctx_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                children=children

                // ARIA
                attr:role="progressbar"
                attr:aria-valuemax=move || max_signal.get().to_string()
                attr:aria-valuemin="0"
                attr:aria-valuenow=move || value_signal.get().map(|v| v.to_string())
                attr:aria-valuetext=move || value_label.get()

                // Data attributes
                attr:data-state=move || progress_state.get().to_string()
                attr:data-value=move || value_signal.get().map(|v| v.to_string())
                attr:data-max=move || max_signal.get().to_string()
            />
        </ProgressProvider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ProgressIndicator
 * -----------------------------------------------------------------------------------------------*/

const INDICATOR_NAME: &'static str = "ProgressIndicator";

#[component]
#[allow(non_snake_case)]
pub fn ProgressIndicator(
    /// If `true`, renders as a child without an extra `<div>`.
    #[prop(into, optional)]
    as_child: MaybeProp<bool>,
    /// Reference to the `<div>`.
    #[prop(into, optional)]
    node_ref: AnyNodeRef,
    /// Child elements, e.g. a styled bar or animated element.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = use_progress_context(INDICATOR_NAME);
    let children = StoredValue::new(children);

    let progress_state = Signal::derive({
        let val = context.value;
        let mx = context.max;
        move || get_progress_state(val.get(), mx.get())
    });

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref

            // Data attributes
            attr:data-state=move || progress_state.get().to_string()
            attr:data-value=move || context.value.get().map(|v| v.to_string())
            attr:data-max=move || context.max.get().to_string()
        >
            {children.with_value(|c| c.as_ref().map(|inner| inner()))}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Helpers
 * -----------------------------------------------------------------------------------------------*/

fn get_progress_state(value: Option<f64>, max_value: f64) -> ProgressState {
    match value {
        Some(v) if v == max_value => ProgressState::Complete,
        Some(_) => ProgressState::Loading,
        None => ProgressState::Indeterminate,
    }
}

/* -------------------------------------------------------------------------------------------------
 * Re-exports
 * -----------------------------------------------------------------------------------------------*/

pub mod primitive {
    pub use super::*;
    pub use Progress as Root;
    pub use ProgressIndicator as Indicator;
}
