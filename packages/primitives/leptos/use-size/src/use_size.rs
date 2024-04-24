use std::{cell::RefCell, rc::Rc};

use leptos::{create_effect, create_signal, html::AnyElement, on_cleanup, NodeRef, ReadSignal};
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

pub fn use_size(element_ref: NodeRef<AnyElement>) -> ReadSignal<Option<Size>> {
    let (size, set_size) = create_signal::<Option<Size>>(None);

    let resize_observer: Rc<RefCell<Option<ResizeObserver>>> = Rc::new(RefCell::new(None));
    let cleanup_resize_observer = resize_observer.clone();

    create_effect(move |_| {
        if let Some(element) = element_ref() {
            // Provide size as early as possible.
            set_size(Some(Size {
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
                            set_size(Some(Size {
                                width: border_size_entry.inline_size(),
                                height: border_size_entry.block_size(),
                            }));
                        }
                    }
                });

            resize_observer.replace(Some(
                ResizeObserver::new(resize_closure.into_js_value().unchecked_ref())
                    .expect("Resize observer should be created."),
            ));

            resize_observer
                .borrow()
                .as_ref()
                .expect("Resize observer should exist.")
                .observe_with_options(
                    element.as_ref(),
                    ResizeObserverOptions::new().box_(ResizeObserverBoxOptions::BorderBox),
                );
        } else {
            // We only want to reset to `None` when the element becomes `None`, not if it changes to another element.
            set_size(None);
        }
    });

    on_cleanup(move || {
        if let Some(resize_observer) = cleanup_resize_observer.take() {
            resize_observer.disconnect();
        }
    });

    size
}
