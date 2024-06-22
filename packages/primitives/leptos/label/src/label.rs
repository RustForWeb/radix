use leptos::{ev::MouseEvent, *};
use radix_leptos_primitive::Primitive;

#[component]
pub fn Label(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] on_mouse_down: Option<Callback<MouseEvent>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Primitive
            element=html::label
            as_child=as_child
            on:mousedown=move |event: MouseEvent| {
                // Only prevent text selection if clicking inside the label itself
                let target: web_sys::Element = event_target(&event);
                if target.closest("button, input, select, textarea").expect("Element should be able to query closest.").is_some() {
                    return
                }

                if let Some(on_mouse_down) = on_mouse_down {
                    Callable::call(&on_mouse_down, event.clone());
                }

                // Prevent text selection when double clicking label
                if !event.default_prevented() && event.detail() > 1 {
                    event.prevent_default();
                }
            }
            attrs=attrs
        >
            {children()}
        </Primitive>
    }
}
