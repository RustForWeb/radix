use std::sync::{Arc, LazyLock};

use dashmap::DashMap;
use send_wrapper::SendWrapper;
use web_sys::{
    DomRect, Element, ResizeObserver, ResizeObserverBoxOptions, ResizeObserverEntry,
    ResizeObserverOptions,
    wasm_bindgen::{JsCast, closure::Closure},
};

type Callback = Arc<dyn Fn(DomRect) + Send + Sync>;

pub type UnobserveCallback = Box<dyn Fn() + Send + Sync>;

struct ElementData {
    callbacks: Vec<Option<Callback>>,
}

static OBSERVED_ELEMENTS: LazyLock<DashMap<String, ElementData>> = LazyLock::new(DashMap::new);

static RESIZE_OBSERVER: LazyLock<SendWrapper<ResizeObserver>> = LazyLock::new(|| {
    let callback: Closure<dyn Fn(Vec<ResizeObserverEntry>)> =
        Closure::new(|entries: Vec<ResizeObserverEntry>| {
            for entry in entries {
                let target = entry.target();
                let id = target.id();

                let callbacks = if let Some(observed_data) = OBSERVED_ELEMENTS.get(&id) {
                    Some(observed_data.callbacks.clone())
                } else {
                    None
                };

                for callback in callbacks.iter().flatten().flatten() {
                    callback(target.get_bounding_client_rect().clone());
                }
            }
        });

    SendWrapper::new(
        ResizeObserver::new(callback.into_js_value().unchecked_ref())
            .expect("Resize observer should be created."),
    )
});

/// uses `ResizeObserver` to observe an element and calls all the registered callbacks when element's
/// size changes
///
/// # Panics
///
/// Panics if failed to acquire locks
#[allow(clippy::significant_drop_tightening)]
pub fn observe_element_rect<C>(element_to_observe: &Element, callback: C) -> UnobserveCallback
where
    C: Fn(DomRect) + 'static + Send + Sync,
{
    let callback: Arc<dyn Fn(DomRect) + 'static + Send + Sync> = Arc::new(callback);
    let mut observed = true;
    let mut callback_idx = 0;
    // TODO: find a better way to identify elements
    let id = {
        let id = element_to_observe.id();
        if id.is_empty() {
            let uuid = uuid::Uuid::new_v4().to_string();
            element_to_observe.set_id(&uuid);

            uuid
        } else {
            id
        }
    };

    {
        let mut observed_data = OBSERVED_ELEMENTS.entry(id.clone()).or_insert_with(|| {
            observed = false;

            ElementData {
                callbacks: Vec::with_capacity(1),
            }
        });

        callback_idx = observed_data.callbacks.len();
        observed_data.callbacks.push(Some(Arc::clone(&callback)));
    }

    if !observed {
        let options = ResizeObserverOptions::new();
        options.set_box(ResizeObserverBoxOptions::BorderBox);

        RESIZE_OBSERVER.observe_with_options(element_to_observe, &options);
    }

    let wrapped_element = SendWrapper::new(element_to_observe.clone());
    Box::new(move || {
        let element = &*wrapped_element;

        let remove = {
            let observed_data = OBSERVED_ELEMENTS.get_mut(&id);

            if let Some(mut observed_data) = observed_data {
                observed_data.callbacks[callback_idx] = None;

                if observed_data
                    .callbacks
                    .iter()
                    .any(std::option::Option::is_some)
                {
                    false
                } else {
                    RESIZE_OBSERVER.unobserve(element);

                    true
                }
            } else {
                false
            }
        };

        if remove {
            OBSERVED_ELEMENTS.remove(&id);
        }
    })
}
