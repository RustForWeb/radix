use std::sync::{Arc, LazyLock, RwLock};

use send_wrapper::SendWrapper;
use web_sys::{
    DomRect, Element, ResizeObserver, ResizeObserverEntry,
    wasm_bindgen::{JsCast, closure::Closure},
};

// TODO: should it be Arc or Rc
type Callback<'a> = Arc<dyn Fn(DomRect) + 'a>;

pub type UnobserveCallback = Box<dyn Fn() + Send + Sync>;

#[derive(Clone)]
struct ObservedData<'a> {
    rect: DomRect,
    callbacks: Vec<Option<Callback<'a>>>,
}

struct ObservedElement {
    element: Element,
    observed_data: ObservedData<'static>,
}

static OBSERVED_ELEMENTS: LazyLock<Arc<RwLock<Vec<SendWrapper<ObservedElement>>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));

static RESIZE_OBSERVER: LazyLock<SendWrapper<ResizeObserver>> = LazyLock::new(|| {
    let callback: Closure<dyn Fn(Vec<ResizeObserverEntry>)> =
        Closure::new(|entries: Vec<ResizeObserverEntry>| {
            for entry in entries {
                let target = entry.target();
                if let Some(observed_element) = find_observed_data(&target) {
                    for callback in observed_element.callbacks.iter().flatten() {
                        callback(entry.target().get_bounding_client_rect().clone());
                    }
                }
            }
        });

    SendWrapper::new(
        ResizeObserver::new(callback.into_js_value().unchecked_ref())
            .expect("Resize observer should be created."),
    )
});

fn find_observed_data(element: &Element) -> Option<ObservedData<'static>> {
    OBSERVED_ELEMENTS
        .read()
        .expect("Read lock should be acquired.")
        .iter()
        .find(|observed_element| observed_element.element == *element)
        .map(|observed_element| observed_element.observed_data.clone())
}

fn find_observed_element_idx(element: &Element) -> Option<usize> {
    OBSERVED_ELEMENTS
        .read()
        .expect("Read lock should be acquired.")
        .iter()
        .position(|observed_element| observed_element.element == *element)
}

/// uses `ResizeObserver` to observe an element and calls all the registered callbacks when element's
/// size changes
///
/// # Panics
///
/// Panics if failed to acquire locks
#[allow(clippy::significant_drop_tightening)]
pub fn observe_element_rect<C>(element_to_observe: &Element, callback: C) -> UnobserveCallback
where
    C: Fn(DomRect) + 'static,
{
    let mut callback_idx = 0;
    let observed_element_idx = find_observed_element_idx(element_to_observe);
    let callback: Arc<dyn Fn(DomRect)> = Arc::new(callback);

    // TODO: what about race conditions, should it be ignored?
    if let Some(idx) = observed_element_idx {
        let mut guard = OBSERVED_ELEMENTS
            .write()
            .expect("Write lock should be acquired.");
        let observed_element = guard.get_mut(idx);

        if let Some(observed_element) = observed_element {
            callback_idx = observed_element.observed_data.callbacks.len();

            observed_element
                .observed_data
                .callbacks
                .push(Some(Arc::clone(&callback)));

            // TODO: does it worth it?
            callback(observed_element.observed_data.rect.clone());
        }
    } else {
        OBSERVED_ELEMENTS
            .write()
            .expect("Write lock should be acquired.")
            .push(SendWrapper::new(ObservedElement {
                element: element_to_observe.clone(),
                observed_data: ObservedData {
                    rect: DomRect::new().expect("DomRect should be created"),
                    callbacks: vec![Some(callback)],
                },
            }));

        RESIZE_OBSERVER.observe(element_to_observe);
    }

    let wrapped_element = SendWrapper::new(element_to_observe.clone());
    Box::new(move || {
        let element = &*wrapped_element;

        let observed_element_idx = find_observed_element_idx(element);

        // TODO: what about race conditions, should it be ignored?
        if let Some(idx) = observed_element_idx {
            let mut guard = OBSERVED_ELEMENTS
                .write()
                .expect("Write lock should be acquired.");
            let observed_element = guard.get_mut(idx);

            if let Some(observed_element) = observed_element {
                observed_element.observed_data.callbacks[callback_idx] = None;

                if !observed_element
                    .observed_data
                    .callbacks
                    .iter()
                    .any(std::option::Option::is_some)
                {
                    RESIZE_OBSERVER.unobserve(element);
                    guard.remove(idx);
                }
            }
        }
    })
}
