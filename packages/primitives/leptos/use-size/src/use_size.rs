use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_observe::observe_element;
use web_sys::ResizeObserverSize;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

pub fn use_size(element_ref: AnyNodeRef) -> ReadSignal<Option<Size>> {
    let (size, set_size) = signal::<Option<Size>>(None);
    let unobserve = Arc::new(Mutex::new(None));
    let unobserve_clone = unobserve.clone();

    Effect::new(move |_| {
        if let Some(element) = element_ref
            .get()
            .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
        {
            // Provide size as early as possible.
            set_size.set(Some(Size {
                width: f64::from(element.offset_width()),
                height: f64::from(element.offset_height()),
            }));

            let cleanup = observe_element(&element, move |entry| {
                let border_size_entry = entry.border_box_size().at(0);

                if let Some(border_size_entry) = border_size_entry.dyn_ref::<ResizeObserverSize>() {
                    set_size.set(Some(Size {
                        width: border_size_entry.inline_size(),
                        height: border_size_entry.block_size(),
                    }));
                }
            });

            unobserve
                .lock()
                .expect("Lock should be acquired.")
                .replace(cleanup);
        } else {
            // We only want to reset to `None` when the element becomes `None`, not if it changes to another element.
            set_size.set(None);
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

    size
}
