use leptos::{ev::MouseEvent, html, prelude::*};
use leptos_maybe_callback::MaybeCallback;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_primitive::Primitive;

#[component]
pub fn Label(
    #[prop(into, optional)] on_mouse_down: MaybeCallback<MouseEvent>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    view! {
        <Primitive
            element=html::label
            as_child=as_child
            node_ref=node_ref
            children=children
            on:mousedown=move |event: MouseEvent| {
                // Only prevent text selection if clicking inside the label itself.
                let target = event_target::<web_sys::Element>(&event);
                if target
                    .closest("button, input, select, textarea")
                    .expect("Element should be able to query closest.")
                    .is_some()
                {
                    return;
                }

                on_mouse_down.run(event.clone());

                // Prevent text selection when double clicking label.
                if !event.default_prevented() && event.detail() > 1 {
                    event.prevent_default();
                }
            }
        />
    }
}
