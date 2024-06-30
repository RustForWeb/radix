use leptos::{create_memo, create_rw_signal, Memo, Signal, SignalGet, SignalSet};

pub fn use_previous<T: Clone + PartialEq>(value: Signal<T>) -> Memo<T> {
    let current = create_rw_signal(value.get());
    let previous = create_rw_signal(value.get());

    create_memo(move |_| {
        let value = value.get();
        let current_value = current.get();
        if current_value != value {
            previous.set(current_value);
            current.set(value.clone());
        }
        previous.get()
    })
}
