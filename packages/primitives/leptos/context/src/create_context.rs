/// Macro to create a context provider and a hook to consume the context.
///
/// # Example
/// ```rust
/// use leptos::prelude::*;
/// use radix_leptos_context::create_context;
///
/// #[derive(Clone)]
/// struct CountContext(i32);
///
/// create_context!(
///     context_type: CountContext,
///     provider: CountProvider,
///     hook: use_count,
///     root: "Count"
/// );
///
/// #[component]
/// fn Counter() -> impl IntoView {
///     let count = use_count("Counter");
///     view! { <div>"Count: "{count.0}</div> }
/// }
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <CountProvider value=CountContext(42)>
///             <Counter/>
///         </CountProvider>
///     }
/// }
/// ```
///
/// # Panics
///
/// The hook will panic if used in a component that is not wrapped in its provider:
/// ```should_panic
/// use leptos::prelude::*;
/// use radix_leptos_context::create_context;
///
/// #[derive(Clone)]
/// struct CountContext(i32);
///
/// create_context!(
///     context_type: CountContext,
///     provider: CountProvider,
///     hook: use_count,
///     root: "Count"
/// );
///
/// #[component]
/// fn BadApp() -> impl IntoView {
///     let count = use_count("BadApp"); // Panics: "`BadApp` must be used within `Count`"
///     view! { <div>{count.0}</div> }
/// }
/// ```
#[macro_export]
macro_rules! create_context {
    (
        context_type: $context_ty:ty,
        provider: $provider:ident,
        hook: $hook:ident,
        root: $root_component_name:expr
    ) => {
        use leptos::prelude::*;

        #[component]
        #[allow(non_snake_case)]
        pub fn $provider(
            value: $context_ty,
            children: TypedChildren<impl IntoView + 'static>,
        ) -> impl IntoView {
            view! { <Provider value=value children=children/> }
        }

        pub fn $hook(component_name: &'static str) -> $context_ty {
            use_context::<$context_ty>()
                .expect(&format!("`{}` must be used within `{}`", component_name, $root_component_name))
                .clone()
        }
    };
}
// TODO: Default context support

