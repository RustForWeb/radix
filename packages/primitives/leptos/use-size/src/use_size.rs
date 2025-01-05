use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    ResizeObserver, ResizeObserverBoxOptions, ResizeObserverEntry, ResizeObserverOptions,
    ResizeObserverSize,
};

#[derive(Clone, Debug)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

pub fn use_size(element_ref: AnyNodeRef) -> ReadSignal<Option<Size>> {
    let (size, set_size) = signal::<Option<Size>>(None);

    let resize_observer: Arc<Mutex<Option<SendWrapper<ResizeObserver>>>> =
        Arc::new(Mutex::new(None));
    let cleanup_resize_observer = resize_observer.clone();

    Effect::new(move |_| {
        if let Some(element) = element_ref
            .get()
            .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
        {
            // Provide size as early as possible.
            set_size.set(Some(Size {
                width: element.offset_width() as f64,
                height: element.offset_height() as f64,
            }));

            let resize_closure: Closure<dyn Fn(Vec<ResizeObserverEntry>)> =
                Closure::new(move |entries: Vec<ResizeObserverEntry>| {
                    if let Some(entry) = entries.first() {
                        let border_size_entry = entry.border_box_size().at(0);

                        if let Some(border_size_entry) =
                            border_size_entry.dyn_ref::<ResizeObserverSize>()
                        {
                            set_size.set(Some(Size {
                                width: border_size_entry.inline_size(),
                                height: border_size_entry.block_size(),
                            }));
                        }
                    }
                });

            *resize_observer.lock().expect("Lock should be acquired.") = Some(SendWrapper::new(
                ResizeObserver::new(resize_closure.into_js_value().unchecked_ref())
                    .expect("Resize observer should be created."),
            ));

            let options = ResizeObserverOptions::new();
            options.set_box(ResizeObserverBoxOptions::BorderBox);

            resize_observer
                .lock()
                .expect("Lock should be acquired.")
                .as_ref()
                .expect("Resize observer should exist.")
                .observe_with_options(element.as_ref(), &options);
        } else {
            // We only want to reset to `None` when the element becomes `None`, not if it changes to another element.
            set_size.set(None);
        }
    });

    on_cleanup(move || {
        if let Some(resize_observer) = cleanup_resize_observer
            .lock()
            .expect("Lock should be acquired.")
            .as_ref()
        {
            resize_observer.disconnect();
        }
    });

    size
}
