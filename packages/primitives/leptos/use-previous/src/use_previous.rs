use leptos::{Memo, RwSignal, Signal, SignalGet, SignalGetUntracked, SignalSetUntracked};

pub fn use_previous<T: Clone + PartialEq>(value: Signal<T>) -> Memo<T> {
    let current = RwSignal::new(value.get_untracked());
    let previous = RwSignal::new(value.get_untracked());

    Memo::new(move |_| {
        let value = value.get();
        let current_value = current.get();
        if current_value != value {
            previous.set_untracked(current_value);
            current.set_untracked(value.clone());
        }
        previous.get()
    })
}
