use std::sync::{Arc, Mutex};

use leptos::{html::ElementType, prelude::*};
use radix_rect::observe_element_rect;
use send_wrapper::SendWrapper;
use web_sys::{DomRect, wasm_bindgen::JsCast};

/// Provides a signal that monitors a node size changes
///
/// # Panics
///
/// Panics if failed to acquire the lock
#[must_use]
pub fn use_rect<E>(element_ref: NodeRef<E>) -> ReadSignal<Option<SendWrapper<DomRect>>>
where
    E: ElementType + JsCast,
    NodeRef<E>: With + Get<Value = Option<E>>,
{
    let (rect, set_rect) = signal::<Option<SendWrapper<DomRect>>>(None);
    let unobserve = Arc::new(Mutex::new(None));
    let unobserve_clone = unobserve.clone();

    Effect::new(move |_| {
        if let Some(element) = element_ref
            .get()
            .and_then(|element| element.dyn_into::<web_sys::Element>().ok())
        {
            *unobserve.lock().expect("Lock should be acquired.") =
                Some(observe_element_rect(&element, move |rect| {
                    set_rect.set(Some(SendWrapper::new(rect)));
                }));
        }
    });

    on_cleanup(move || {
        if let Some(unobserve) = unobserve_clone
            .lock()
            .expect("Lock should be acquired.")
            .as_ref()
        {
            unobserve();
        }
    });

    rect
}
