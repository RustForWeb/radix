use leptos::{
    ev::Event,
    html::{ElementType, HtmlElement},
    prelude::*,
    tachys::html::node_ref::NodeRefContainer,
    wasm_bindgen::JsCast,
};
use leptos_node_ref::{AnyNodeRef, any_node_ref};
use leptos_typed_fallback_show::TypedFallbackShow;

#[component]
pub fn Primitive<E, C>(
    element: fn() -> HtmlElement<E, (), ()>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
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
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
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
        if let Some(original) = &original_handler {
            original.run(event.clone());
        }

        if !check_default_prevented || !event.clone().into().default_prevented() {
            if let Some(our) = &our_handler {
                our.run(event);
            }
        }
    }
}
