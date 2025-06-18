use std::sync::LazyLock;

use send_wrapper::SendWrapper;
use web_sys::js_sys;
use web_sys::wasm_bindgen::prelude::*;
use web_sys::{
    Element, ResizeObserver, ResizeObserverBoxOptions, ResizeObserverEntry, ResizeObserverOptions,
};

pub type UnobserveCallback = Box<dyn Fn() + Send + Sync>;

static OBSERVED_ELEMENTS: LazyLock<SendWrapper<js_sys::Map>> =
    LazyLock::new(|| SendWrapper::new(js_sys::Map::new()));

static RESIZE_OBSERVER: LazyLock<SendWrapper<ResizeObserver>> = LazyLock::new(|| {
    let callback: Closure<dyn Fn(Vec<ResizeObserverEntry>)> =
        Closure::new(|entries: Vec<ResizeObserverEntry>| {
            for entry in entries {
                let target = entry.target();

                let observed_data = OBSERVED_ELEMENTS.get(&target);

                // cannot happen
                if observed_data == JsValue::UNDEFINED {
                    // RESIZE_OBSERVER.unobserve(&target);
                } else {
                    let observed_data = observed_data
                        .dyn_ref::<js_sys::Object>()
                        .expect("Object should be of type ObservedData");

                    js_sys::Reflect::set(observed_data, &"entry".into(), &entry)
                        .expect("Should be able to set entry");

                    let callbacks = js_sys::Reflect::get(observed_data, &"callbacks".into())
                        .expect("Object should have callbacks array")
                        .dyn_into::<js_sys::Array>()
                        .expect("Object should be of type array");

                    callbacks.for_each(&mut |obj, _, _| {
                        if let Some(callback) = obj.dyn_ref::<js_sys::Function>() {
                            callback
                                .call1(&JsValue::NULL, &entry)
                                .expect("callback should be called");
                        }
                    });
                }
            }
        });

    SendWrapper::new(
        ResizeObserver::new(callback.into_js_value().unchecked_ref())
            .expect("Resize observer should be created."),
    )
});

pub fn observe_element<C>(element_to_observe: &Element, callback: C) -> UnobserveCallback
where
    C: Fn(ResizeObserverEntry) + 'static + Send + Sync,
{
    let callback: Closure<dyn Fn(ResizeObserverEntry)> =
        Closure::new(move |entry: ResizeObserverEntry| callback(entry));
    let callback = callback.into_js_value();

    let observed_data = OBSERVED_ELEMENTS.get(element_to_observe);
    if observed_data == JsValue::UNDEFINED {
        let obj = js_sys::Object::new();
        let callbacks = js_sys::Array::new();
        callbacks.push(callback.unchecked_ref());

        js_sys::Reflect::set(&obj, &"entry".into(), &JsValue::NULL)
            .expect("Should be able to set entry");
        js_sys::Reflect::set(&obj, &"callbacks".into(), &callbacks.into())
            .expect("Should be able to set callbacks");

        OBSERVED_ELEMENTS.set(element_to_observe, &obj);

        let options = ResizeObserverOptions::new();
        options.set_box(ResizeObserverBoxOptions::BorderBox);

        RESIZE_OBSERVER.observe_with_options(element_to_observe, &options);
    } else {
        let observed_data = observed_data
            .dyn_ref::<js_sys::Object>()
            .expect("observed data type should be object");

        let callbacks = js_sys::Reflect::get(observed_data, &"callbacks".into())
            .expect("Object should have callbacks array")
            .dyn_into::<js_sys::Array>()
            .expect("Object should be of type array");

        callbacks.push(callback.unchecked_ref());

        if let Some(callback) = callbacks
            .get(callbacks.length() - 1)
            .dyn_ref::<js_sys::Function>()
        {
            let entry = js_sys::Reflect::get(observed_data, &"entry".into())
                .expect("ObservedData should have entry");

            if entry != JsValue::NULL {
                callback
                    .call1(
                        &JsValue::NULL,
                        entry
                            .dyn_ref::<ResizeObserverEntry>()
                            .expect("entry should be of type ResizeObserverEntry"),
                    )
                    .expect("callback should be called");
            }
        }
    }

    let wrapped_element = SendWrapper::new(element_to_observe.clone());
    let wrapped_closure = SendWrapper::new(callback);
    Box::new(move || {
        let element = &*wrapped_element;
        let callback = &*wrapped_closure;
        let observed_data = OBSERVED_ELEMENTS.get(element);

        if observed_data == JsValue::UNDEFINED {
            return;
        }

        let observed_data = observed_data
            .dyn_ref::<js_sys::Object>()
            .expect("observed data type should be object");

        let callbacks = js_sys::Reflect::get(observed_data, &"callbacks".into())
            .expect("Object should have callbacks array")
            .dyn_into::<js_sys::Array>()
            .expect("Object should be of type array");

        let index = callbacks.index_of(callback.unchecked_ref(), 0);

        if let Ok(index) = u32::try_from(index) {
            callbacks.splice(index, 1, &JsValue::NULL);
        }

        if callbacks.length() == 0 {
            RESIZE_OBSERVER.unobserve(element);
            OBSERVED_ELEMENTS.delete(element);
        }
    })
}
