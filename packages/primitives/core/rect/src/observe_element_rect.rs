use std::{
    hash::Hash,
    sync::{Arc, LazyLock},
};

use dashmap::DashMap;
use send_wrapper::SendWrapper;
use web_sys::{
    DomRect, Element, ResizeObserver, ResizeObserverEntry,
    js_sys::Object,
    wasm_bindgen::{JsCast, JsValue, closure::Closure},
};

struct ObservedData {
    rect: DomRect,
    callbacks: Vec<Box<dyn Fn(DomRect)>>,
}

#[derive(Debug, Clone)]
struct ElementWrapper {
    pub inner: SendWrapper<Element>,
}

impl PartialEq for ElementWrapper {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl Eq for ElementWrapper {}

impl Hash for ElementWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let res = self.inner.unchecked_ref::<Object>();
        let val: &JsValue = res.as_ref();
        Object::is(val, val);
        // self.inner.hash(state);
    }
}

static OBSERVED_ELEMENTS: LazyLock<Arc<DashMap<ElementWrapper, SendWrapper<ObservedData>>>> =
    LazyLock::new(|| Arc::new(DashMap::new()));

static RESIZE_OBSERVER: LazyLock<SendWrapper<ResizeObserver>> = LazyLock::new(|| {
    let callback: Closure<dyn Fn(Vec<ResizeObserverEntry>)> =
        Closure::new(|entries: Vec<ResizeObserverEntry>| {
            for entry in entries {
                let target = entry.target();
                let data = OBSERVED_ELEMENTS.get(&ElementWrapper {
                    inner: SendWrapper::new(target),
                });

                if let Some(data) = data {
                    for callback in &data.callbacks {
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

#[allow(clippy::significant_drop_tightening)]
pub fn observe_element_rect<C>(
    element_to_observe: &Element,
    callback: C,
) -> Box<dyn Fn() + Send + Sync>
where
    C: Fn(DomRect) + 'static,
{
    let safe_element = SendWrapper::new(element_to_observe.clone());
    let observed_data = OBSERVED_ELEMENTS.get_mut(&ElementWrapper {
        inner: safe_element.clone(),
    });

    if let Some(mut observed_data) = observed_data {
        // TODO: is this optimal or needed?
        callback(observed_data.rect.clone());

        observed_data
            .callbacks
            .push(Box::new(callback) as Box<dyn Fn(DomRect)>);
    } else {
        OBSERVED_ELEMENTS.insert(
            ElementWrapper {
                inner: SendWrapper::new(element_to_observe.clone()),
            },
            SendWrapper::new(ObservedData {
                rect: DomRect::new().expect("DomRect should be created"),
                callbacks: vec![Box::new(callback)],
            }),
        );

        RESIZE_OBSERVER.observe(element_to_observe);
    }

    Box::new(move || {
        let element = &*safe_element;
        RESIZE_OBSERVER.unobserve(element);
    })
}
