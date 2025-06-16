use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_rect::observe_element_rect;
use send_wrapper::SendWrapper;
use web_sys::{DomRect, wasm_bindgen::JsCast};

#[must_use]
pub fn use_rect(element_ref: AnyNodeRef) -> ReadSignal<Option<SendWrapper<DomRect>>> {
    let (rect, set_rect) = signal::<Option<SendWrapper<DomRect>>>(None);
    let unobserve: Arc<Mutex<Option<Box<dyn Fn() + Send + Sync>>>> = Arc::new(Mutex::new(None));
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
