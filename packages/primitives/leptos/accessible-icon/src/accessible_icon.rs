use leptos::attr::{aria_hidden, custom::custom_attribute};
use leptos::prelude::*;
use radix_leptos_visually_hidden::primitive as VisuallyHidden;

/* -------------------------------------------------------------------------------------------------
 * AccessibleIcon
 * -----------------------------------------------------------------------------------------------*/

#[allow(unused)]
const NAME: &str = "AccessibleIcon";

/// A minimal port of Radix UI’s AccessibleIcon to Leptos.
///
/// - Uses [`TypedChildren`](leptos::prelude::TypedChildren) so attributes can pass through via `add_any_attr`,
///   since `ChildrenFragment` and `AnyView` don’t support this in Leptos 0.7.3.
/// - Applies `aria-hidden="true"` and `focusable="false"` to hide the icon from screen readers (the label is announced).
/// - Does not enforce a single child or clone/modify SVGs like React’s `cloneElement`.
#[component]
#[allow(non_snake_case)]
pub fn AccessibleIcon(
    children: TypedChildren<impl IntoView + 'static>,
    /// A label announced by screen readers, hidden visually.
    #[prop(into, optional)] label: MaybeProp<String>,
) -> impl IntoView {
    let label = Signal::derive(move || label.get());
    view! {
        <>
            {children
                .into_inner()()
                .add_any_attr(aria_hidden("true"))
                .add_any_attr(custom_attribute("focusable", "false"))}
            <Show when=move || label.get().is_some()>
                <VisuallyHidden::Root>{label.get().unwrap_or_default()}</VisuallyHidden::Root>
            </Show>
        </>
    }
}


/* -----------------------------------------------------------------------------------------------*/

pub mod primitive {
    pub use super::*;
    pub use AccessibleIcon as Root;
}
