use leptos::{html::AnyElement, *};
use leptos_portal::LeptosPortal;
use radix_leptos_primitive::Primitive;

#[component]
pub fn Portal(
    #[prop(into, optional)] container: MaybeProp<web_sys::Element>,
    #[prop(optional)] container_ref: NodeRef<AnyElement>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let attrs = StoredValue::new(attrs);
    let children = StoredValue::new(children);

    view! {
        <LeptosPortal mount=container mount_ref=container_ref>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attrs=attrs.get_value()
            >
                {children.with_value(|children| children())}
            </Primitive>
        </LeptosPortal>
    }
}

/// Based on [`leptos::Portal`].
mod leptos_portal {
    use cfg_if::cfg_if;
    use leptos::{component, html::AnyElement, ChildrenFn, MaybeProp, NodeRef};
    use leptos_dom::IntoView;

    /// Renders components somewhere else in the DOM.
    ///
    /// Useful for inserting modals and tooltips outside of a cropping layout.
    /// If no mount point is given, the portal is inserted in `document.body`;
    #[cfg_attr(
        any(debug_assertions, feature = "ssr"),
        tracing::instrument(level = "trace", skip_all)
    )]
    #[component]
    pub fn LeptosPortal(
        /// Target element where the children will be appended
        #[prop(into, optional)]
        mount: MaybeProp<web_sys::Element>,
        #[prop(optional)] mount_ref: NodeRef<AnyElement>,
        /// The children to teleport into the `mount` element
        children: ChildrenFn,
    ) -> impl IntoView {
        cfg_if! { if #[cfg(all(target_arch = "wasm32", any(feature = "hydrate", feature = "csr")))] {
            use leptos::{on_cleanup, Effect, RwSignal, Signal, SignalGet, SignalSet, StoredValue};
            use leptos_dom::{document, Mountable};
            use web_sys::wasm_bindgen::JsCast;

            let children = StoredValue::new(children);

            let mount = Signal::derive(move || {
                mount_ref
                    .get()
                    .map(|mount| {
                        let element: &web_sys::HtmlElement = &mount;
                        element.clone().unchecked_into::<web_sys::Element>()
                    })
                    .or_else(|| mount.get())
                    .unwrap_or_else(|| document().body().expect("body to exist").into())
            });

            let current_mount: RwSignal<Option<web_sys::Element>> = RwSignal::new(None);
            let current_nodes: RwSignal<Option<Vec<web_sys::Node>>> = RwSignal::new(None);

            let remove_nodes = move |current_mount: &web_sys::Element | {
                if let Some(current_nodes) = current_nodes.get() {
                    for current_node in current_nodes {
                        current_mount.remove_child(&current_node).expect("child to be removed");
                    }
                }
            };

            Effect::new(move |_| {
                let mount = mount.get();
                if current_mount.get().as_ref() != Some(&mount) {
                    if let Some(current_mount) = current_mount.get() {
                        remove_nodes(&current_mount);
                    }
                    current_mount.set(Some(mount));
                }
            });

            Effect::new(move |_| {
                if let Some(current_mount) = current_mount.get() {
                    remove_nodes(&current_mount);

                    let node = children.with_value(|children| children().into_view().get_mountable_node());
                    if let Some(fragment) = node.dyn_ref::<web_sys::DocumentFragment>() {
                        let mut nodes: Vec<web_sys::Node> = vec![];
                        for index in 0..fragment.children().length() {
                            nodes.push(fragment.children().item(index).expect("child to exist").into());
                        }

                        current_mount.append_child(&node).expect("child to be appended");
                        current_nodes.set(Some(nodes));
                    } else {
                        current_nodes.set(Some(vec![current_mount.append_child(&node).expect("child to be appended")]));
                    }
                }
            });

            on_cleanup(move || {
                if let Some(current_mount) = current_mount.get() {
                    remove_nodes(&current_mount);
                }
            });
        } else {
            let _ = mount;
            let _ = mount_ref;
            let _ = children;
        }}
    }
}
