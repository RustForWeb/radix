use leptos::{attribute_interceptor::AttributeInterceptor, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_primitive::Primitive;

#[component]
pub fn AspectRatio(
    #[prop(into, optional, default = 1.0.into())] ratio: Signal<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <AttributeInterceptor let:attrs>
            <div
                // Ensures inner element is contained
                style:position="relative"
                // Ensures padding bottom trick maths works
                style:width="100%"
                style:padding-bottom=move || format!("{}%", 100.0 / ratio.get())
                data-radix-aspect-ratio-wrapper=""
            >
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=node_ref
                    // Ensures children expand in ratio
                    style:position="absolute"
                    style:top="0px"
                    style:right="0px"
                    style:bottom="0px"
                    style:left="0px"
                    {..attrs}
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </div>
        </AttributeInterceptor>
    }
}
