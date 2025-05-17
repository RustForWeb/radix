use yew::prelude::*;

pub fn compose_callbacks<E: Clone + Into<Event> + 'static>(
    original_event_handler: Option<Callback<E>>,
    our_event_handler: Option<Callback<E>>,
    check_for_default_prevented: Option<bool>,
) -> Callback<E> {
    let check_for_default_prevented = check_for_default_prevented.unwrap_or(true);

    Callback::from(move |event: E| {
        if let Some(original_event_handler) = &original_event_handler {
            original_event_handler.emit(event.clone());
        }

        if (!check_for_default_prevented || !event.clone().into().default_prevented())
            && let Some(our_event_handler) = &our_event_handler
        {
            our_event_handler.emit(event);
        }
    })
}
