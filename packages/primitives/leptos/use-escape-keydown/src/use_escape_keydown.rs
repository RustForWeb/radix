use std::sync::Arc;

use leptos::{ev::KeyboardEvent, prelude::*};
use send_wrapper::SendWrapper;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    AddEventListenerOptions, Document, EventListenerOptions,
};

/// Listens for when the escape key is down.
pub fn use_escape_keydown(
    on_escape_key_down: Option<Callback<KeyboardEvent>>,
    owner_document: Option<Document>,
) {
    let owner_document = StoredValue::new(SendWrapper::new(owner_document.unwrap_or(document())));

    type HandleKeyDown = dyn Fn(KeyboardEvent);
    let handle_key_down: Arc<SendWrapper<Closure<HandleKeyDown>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: KeyboardEvent| {
            if event.key() == "Escape" {
                if let Some(on_escape_key_down) = on_escape_key_down {
                    on_escape_key_down.run(event);
                }
            }
        }),
    ));

    Effect::new({
        let handle_key_down = handle_key_down.clone();

        move |_| {
            let options = AddEventListenerOptions::new();
            options.set_capture(true);

            owner_document
                .get_value()
                .add_event_listener_with_callback_and_add_event_listener_options(
                    "keydown",
                    (*handle_key_down).as_ref().unchecked_ref(),
                    &options,
                )
                .expect("Key down event listener should be added.");
        }
    });

    on_cleanup(move || {
        let options = EventListenerOptions::new();
        options.set_capture(true);

        owner_document
            .get_value()
            .remove_event_listener_with_callback_and_event_listener_options(
                "keydown",
                (*handle_key_down).as_ref().unchecked_ref(),
                &options,
            )
            .expect("Key down event listener should be removed.");
    });
}
