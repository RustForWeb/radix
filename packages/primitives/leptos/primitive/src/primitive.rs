use leptos::{
    attr::Attribute,
    either::Either,
    ev::Event,
    html::{ElementType, HtmlElement},
    prelude::*,
    tachys::html::{class::IntoClass, node_ref::NodeRefContainer, style::IntoStyle},
};
use leptos_node_ref::{any_node_ref, AnyNodeRef};
use wasm_bindgen::JsCast;

/// We need our own show instead of leptos' Show because attribute spreading does not work
/// across AnyView as of 0.7.3, which is required here.
#[component]
#[allow(non_snake_case)]
pub fn TypedFallbackShow<F, IV, W, C>(
    children: TypedChildrenFn<C>,
    when: W,
    fallback: F,
) -> impl IntoView
where
    W: Fn() -> bool + Send + Sync + 'static,
    F: Fn() -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
    C: IntoView + 'static,
{
    let memoized_when = ArcMemo::new(move |_| when());
    let children = children.into_inner();

    move || match memoized_when.get() {
        true => Either::Left(children()),
        false => Either::Right(fallback().into_view()),
    }
}

#[component]
pub fn Primitive<E, C>(
    element: fn() -> HtmlElement<E, (), ()>,
    children: TypedChildrenFn<C>,
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    #[prop(optional, into)] node_ref: AnyNodeRef,
) -> impl IntoView
where
    E: ElementType + 'static,
    C: IntoView + 'static,
    View<C>: RenderHtml,
    HtmlElement<E, (), ()>: ElementChild<View<C>>,
    <HtmlElement<E, (), ()> as ElementChild<View<C>>>::Output: IntoView,
    <E as ElementType>::Output: JsCast,
    AnyNodeRef: NodeRefContainer<E>,
{
    let children = StoredValue::new(children.into_inner());

    view! {
        <TypedFallbackShow
            when=move || as_child.get().unwrap_or_default()
            fallback=move || {
                element().child(children.with_value(|children| children())).add_any_attr(any_node_ref(node_ref))
            }
        >
            {children.with_value(|children| children()).add_any_attr(any_node_ref(node_ref))}
        </TypedFallbackShow>
    }
}

#[component]
pub fn VoidPrimitive<E, C>(
    element: fn() -> HtmlElement<E, (), ()>,
    children: TypedChildrenFn<C>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView
where
    E: ElementType + 'static,
    C: IntoView + 'static,
    View<C>: RenderHtml,
    <E as ElementType>::Output: JsCast,
    AnyNodeRef: NodeRefContainer<E>,
{
    let children = StoredValue::new(children.into_inner());
    view! {
        <TypedFallbackShow
            when=move || as_child.get().unwrap_or_default()
            fallback=move || { element().add_any_attr(any_node_ref(node_ref)) }
        >
            {children.with_value(|children| children()).add_any_attr(any_node_ref(node_ref))}
        </TypedFallbackShow>
    }
}

pub fn compose_callbacks<E>(
    original_handler: Option<Callback<E>>,
    our_handler: Option<Callback<E>>,
    check_default_prevented: Option<bool>,
) -> impl Fn(E)
where
    E: Clone + Into<Event> + 'static,
{
    let check_default_prevented = check_default_prevented.unwrap_or(true);

    move |event: E| {
        // Run original handler first, matching TypeScript behavior
        if let Some(original) = &original_handler {
            original.run(event.clone());
        }

        // Only run our handler if default wasn't prevented (when checking is enabled)
        if !check_default_prevented || !event.clone().into().default_prevented() {
            if let Some(our) = &our_handler {
                our.run(event);
            }
        }
    }
}