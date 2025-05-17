use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use leptos_portal::LeptosPortal;
use radix_leptos_primitive::Primitive;
use send_wrapper::SendWrapper;

#[component]
pub fn Portal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    // TODO: pass attrs to primitive
    view! {
        // <AttributeInterceptor let:attrs>
            <LeptosPortal mount=container mount_ref=container_ref>
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref={node_ref}
                    // {..attrs}
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </LeptosPortal>
        // </AttributeInterceptor>
    }
}

/// Based on [`leptos::Portal`].
mod leptos_portal {
    use std::sync::Arc;

    use leptos::prelude::{
        Effect, Get, IntoView, MaybeProp, Owner, RwSignal, Set, Signal, TypedChildrenFn, component,
        mount_to, untrack,
    };
    use leptos_dom::helpers::document;
    use leptos_node_ref::AnyNodeRef;
    use send_wrapper::SendWrapper;

    /// Renders components somewhere else in the DOM.
    ///
    /// Useful for inserting modals and tooltips outside of a cropping layout.
    /// If no mount point is given, the portal is inserted in `document.body`.
    #[component]
    pub fn LeptosPortal<V>(
        /// Target element where the children will be appended
        #[prop(into, optional)]
        mount: MaybeProp<SendWrapper<web_sys::Element>>,
        #[prop(optional)] mount_ref: AnyNodeRef,
        /// The children to teleport into the `mount` element
        children: TypedChildrenFn<V>,
    ) -> impl IntoView
    where
        V: IntoView + 'static,
    {
        if cfg!(target_arch = "wasm32")
            && Owner::current_shared_context()
                .map(|sc| sc.is_browser())
                .unwrap_or(true)
        {
            use web_sys::wasm_bindgen::JsCast;

            let mount = Signal::derive(move || {
                mount_ref
                    .get()
                    .map(|mount| SendWrapper::new(mount.unchecked_into::<web_sys::Element>()))
                    .or_else(|| mount.get())
                    .unwrap_or_else(|| {
                        SendWrapper::new(document().body().expect("body to exist").into())
                    })
            });
            let children = children.into_inner();

            let current_mount: RwSignal<Option<SendWrapper<web_sys::Element>>> =
                RwSignal::new(None);

            Effect::new(move |_| {
                let mount = mount.get();

                if current_mount.get().as_deref() != Some(&*mount) {
                    current_mount.set(Some(mount));
                }
            });

            Effect::new(move |_| {
                if let Some(current_mount) = current_mount.get() {
                    let handle =
                        SendWrapper::new(mount_to((*current_mount).clone().unchecked_into(), {
                            let children = Arc::clone(&children);
                            move || untrack(|| children())
                        }));

                    Owner::on_cleanup({
                        move || {
                            let handle = handle.take();
                            drop(handle);
                        }
                    })
                }
            });
        }
    }
}
