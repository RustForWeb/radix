use std::sync::LazyLock;

use send_wrapper::SendWrapper;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Map;
use web_sys::{
    DomRect, Element, ResizeObserver, ResizeObserverEntry, ResizeObserverOptions, js_sys,
};
use web_sys::{ResizeObserverBoxOptions, wasm_bindgen};

pub type UnobserveCallback = Box<dyn Fn() + Send + Sync>;

static OBSERVED_ELEMENTS: LazyLock<SendWrapper<Map>> =
    LazyLock::new(|| SendWrapper::new(Map::new()));

static RESIZE_OBSERVER: LazyLock<SendWrapper<ResizeObserver>> = LazyLock::new(|| {
    let callback: Closure<dyn Fn(Vec<ResizeObserverEntry>)> =
        Closure::new(|entries: Vec<ResizeObserverEntry>| {
            for entry in entries {
                let target = entry.target();

                let observed_data = OBSERVED_ELEMENTS.get(&target);

                if observed_data == JsValue::UNDEFINED {
                    RESIZE_OBSERVER.unobserve(&target);
                } else {
                    let observed_data = OBSERVED_ELEMENTS
                        .get(&target)
                        .dyn_into::<js_sys::Object>()
                        .expect("Object should be of type ObservedData");

                    js_sys::Reflect::set(
                        &observed_data,
                        &"rect".into(),
                        &target.get_bounding_client_rect(),
                    )
                    .expect("Should be able to set rect");

                    let callbacks = js_sys::Reflect::get(&observed_data, &"callbacks".into())
                        .expect("Object should have callbacks array")
                        .dyn_into::<js_sys::Array>()
                        .expect("Object should be of type array");

                    callbacks.for_each(&mut |obj, _, _| {
                        if let Ok(callback) = obj.dyn_into::<js_sys::Function>() {
                            callback
                                .call1(&JsValue::NULL, &target.get_bounding_client_rect())
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

pub fn observe_element_rect<C>(element_to_observe: &Element, callback: C) -> UnobserveCallback
where
    C: Fn(DomRect) + 'static + Send + Sync,
{
    let observed_data = OBSERVED_ELEMENTS.get(element_to_observe);
    let callback: Closure<dyn Fn(DomRect)> = Closure::new(move |rect: DomRect| callback(rect));
    let callback = callback.into_js_value();

    if observed_data == JsValue::UNDEFINED {
        let obj = js_sys::Object::new();
        let callbacks = js_sys::Array::new();
        callbacks.push(callback.unchecked_ref());

        js_sys::Reflect::set(&obj, &"rect".into(), &JsValue::NULL)
            .expect("Should be able to set rect");
        js_sys::Reflect::set(&obj, &"callbacks".into(), &callbacks.into())
            .expect("Should be able to set callbacks");

        OBSERVED_ELEMENTS.set(element_to_observe, &obj);

        let options = ResizeObserverOptions::new();
        options.set_box(ResizeObserverBoxOptions::BorderBox);

        RESIZE_OBSERVER.observe_with_options(element_to_observe, &options);
    } else {
        let observed_data = observed_data
            .dyn_into::<js_sys::Object>()
            .expect("observed data type should be object");

        let callbacks = js_sys::Reflect::get(&observed_data, &"callbacks".into())
            .expect("Object should have callbacks array")
            .dyn_into::<js_sys::Array>()
            .expect("Object should be of type array");

        callbacks.push(callback.unchecked_ref());

        if let Ok(callback) = callbacks
            .get(callbacks.length() - 1)
            .dyn_into::<js_sys::Function>()
        {
            callback
                .call1(
                    &JsValue::NULL,
                    &js_sys::Reflect::get(&observed_data, &"rect".into())
                        .expect("ObservedData should have rect")
                        .dyn_into::<DomRect>()
                        .expect("rect should be of type DOMRect"),
                )
                .expect("callback should be called");
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
            .dyn_into::<js_sys::Object>()
            .expect("observed data type should be object");

        let callbacks = js_sys::Reflect::get(&observed_data, &"callbacks".into())
            .expect("Object should have callbacks array")
            .dyn_into::<js_sys::Array>()
            .expect("Object should be of type array");

        let index = callbacks.index_of(callback.unchecked_ref(), 0);

        if index > -1 {
            callbacks.splice(index as u32, 1, &JsValue::NULL);
        }

        if callbacks.length() == 0 {
            OBSERVED_ELEMENTS.delete(element);
        }
    })
}
