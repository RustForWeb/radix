use web_sys::Event;

pub fn compose_event_handlers<E: Clone + Into<Event>>(
    original_event_handler: Option<fn(E)>,
    our_event_handler: Option<fn(E)>,
    check_for_default_prevented: Option<bool>,
) -> impl Fn(E) {
    let check_for_default_prevented = check_for_default_prevented.unwrap_or(true);

    move |event: E| {
        if let Some(original_event_handler) = original_event_handler {
            original_event_handler(event.clone());
        }

        if !check_for_default_prevented || !event.clone().into().default_prevented() {
            if let Some(our_event_handler) = our_event_handler {
                our_event_handler(event);
            }
        }
    }
}
