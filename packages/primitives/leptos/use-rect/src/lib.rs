use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_observe::observe_element;
use send_wrapper::SendWrapper;
use web_sys::{DomRect, wasm_bindgen::JsCast};

/// Provides a signal that monitors a node size changes
///
/// # Panics
///
/// Panics if failed to acquire the lock
#[must_use]
pub fn use_rect(element_ref: AnyNodeRef) -> ReadSignal<Option<SendWrapper<DomRect>>> {
    let (rect, set_rect) = signal::<Option<SendWrapper<DomRect>>>(None);
    let unobserve = Arc::new(Mutex::new(None));
    let unobserve_clone = unobserve.clone();

    Effect::new(move |_| {
        if let Some(element) = element_ref
            .get()
            .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let cleanup = observe_element(&element, move |entry| {
                set_rect.set(Some(SendWrapper::new(
                    entry.target().get_bounding_client_rect(),
                )));
            });

            unobserve
                .lock()
                .expect("Lock should be acquired.")
                .replace(cleanup);
        }
    });

    on_cleanup(move || {
        if let Some(cleanup) = unobserve_clone
            .lock()
            .expect("Lock should be acquired.")
            .as_ref()
        {
            cleanup();
        }
    });

    rect
}
