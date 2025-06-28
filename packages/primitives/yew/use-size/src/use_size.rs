use web_sys::{
    ResizeObserver, ResizeObserverBoxOptions, ResizeObserverEntry, ResizeObserverOptions,
    ResizeObserverSize,
    wasm_bindgen::{JsCast, closure::Closure},
};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[hook]
pub fn use_size(element_ref: NodeRef) -> UseStateHandle<Option<Size>> {
    let size: UseStateHandle<Option<Size>> = use_state_eq(|| None);

    use_effect_with(element_ref, {
        let size = size.clone();

        |element_ref| {
            let mut cleanup: Option<Box<dyn Fn()>> = None;

            if let Some(node) = element_ref.get() {
                let element = node
                    .dyn_into::<web_sys::HtmlElement>()
                    .expect("Reference element should be an HtmlElement.");

                // Provide size as early as possible.
                size.set(Some(Size {
                    width: f64::from(element.offset_width()),
                    height: f64::from(element.offset_height()),
                }));

                let resize_closure: Closure<dyn Fn(Vec<ResizeObserverEntry>)> =
                    Closure::new(move |entries: Vec<ResizeObserverEntry>| {
                        if let Some(entry) = entries.first() {
                            let border_size_entry = entry.border_box_size().at(0);

                            if let Some(border_size_entry) =
                                border_size_entry.dyn_ref::<ResizeObserverSize>()
                            {
                                size.set(Some(Size {
                                    width: border_size_entry.inline_size(),
                                    height: border_size_entry.block_size(),
                                }));
                            }
                        }
                    });

                let resize_observer =
                    ResizeObserver::new(resize_closure.into_js_value().unchecked_ref())
                        .expect("Resize observer should be created.");

                let options = ResizeObserverOptions::new();
                options.set_box(ResizeObserverBoxOptions::BorderBox);

                resize_observer.observe_with_options(element.as_ref(), &options);

                cleanup = Some(Box::new(move || {
                    resize_observer.unobserve(&element);
                }));
            } else {
                // We only want to reset to `None` when the element becomes `None`, not if it changes to another element.
                size.set(None);
            }

            move || {
                if let Some(cleanup) = cleanup {
                    cleanup();
                }
            }
        }
    });

    size
}
