use std::rc::Rc;

use leptos::{
    create_effect, document,
    ev::KeyboardEvent,
    on_cleanup,
    web_sys::{
        wasm_bindgen::{closure::Closure, JsCast},
        AddEventListenerOptions, Document,
    },
    Callable, Callback, StoredValue,
};

/// Listens for when the escape key is down.
pub fn use_escape_keydown(
    on_escape_key_down: Option<Callback<KeyboardEvent>>,
    owner_document: Option<Document>,
) {
    let owner_document = StoredValue::new(owner_document.unwrap_or(document()));

    let handle_key_down: Rc<Closure<dyn Fn(KeyboardEvent)>> =
        Rc::new(Closure::new(move |event: KeyboardEvent| {
            if event.key() == "Escape" {
                if let Some(on_escape_key_down) = on_escape_key_down {
                    on_escape_key_down.call(event);
                }
            }
        }));
    let cleanup_handle_key_down = handle_key_down.clone();

    create_effect(move |_| {
        owner_document
            .get_value()
            .add_event_listener_with_callback_and_add_event_listener_options(
                "keydown",
                (*handle_key_down).as_ref().unchecked_ref(),
                AddEventListenerOptions::new().capture(true),
            )
            .expect("Key down event listener should be added.");
    });

    on_cleanup(move || {
        // TODO: change to options once web_sys supports it
        owner_document
            .get_value()
            .remove_event_listener_with_callback_and_bool(
                "keydown",
                (*cleanup_handle_key_down).as_ref().unchecked_ref(),
                true,
            )
            .expect("Key down event listener should be removed.");
    });
}
