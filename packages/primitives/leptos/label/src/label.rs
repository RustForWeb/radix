use leptos::{ev::MouseEvent, html, prelude::*};
use radix_leptos_primitive::Primitive;
use leptos_node_ref::prelude::*;

/* -------------------------------------------------------------------------------------------------
 * Label
 * -----------------------------------------------------------------------------------------------*/

const NAME: &str = "Label";

#[component]
#[allow(non_snake_case)]
pub fn Label(
    children: TypedChildrenFn<impl IntoView + 'static>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] on_mouse_down: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <Primitive
            children={children}
            element=html::label
            as_child=as_child
            on:mousedown=move |event: MouseEvent| {
                // only prevent text selection if clicking inside the label itself
                let target: web_sys::Element = event_target(&event);
                if target
                    .closest("button, input, select, textarea")
                    .expect("Element should be able to query closest.")
                    .is_some()
                {
                    return;
                }

                if let Some(on_mouse_down) = on_mouse_down {
                    on_mouse_down.run( event.clone());
                }
                // prevent text selection when double-clicking label
                if !event.default_prevented() && event.detail() > 1 {
                    event.prevent_default();
                }
            }
            node_ref=node_ref
        />
    }
}

/* -----------------------------------------------------------------------------------------------*/

pub mod primitive {
    pub use super::*;
    pub use Label as Root;
}
