use leptos::prelude::*;

/// A Leptos hook that returns the *previous* value of a reactive Signal.
/// Similar to React's `usePrevious`, it only updates when `value` actually changes.
pub fn use_previous<T: Clone + PartialEq + Send + Sync + 'static>(value: Signal<T>) -> Memo<T> {
    // Keep a store of (current, previous)
    let container = StoredValue::new((value.get_untracked(), value.get_untracked()));

    // Return a Memo that recalculates the previous value only when `value` changes
    Memo::new(move |_| {
        let current_value = value.get_untracked();
        let (stored_current, stored_previous) = container.get_value();

        if stored_current != current_value {
            container.set_value((current_value.clone(), stored_current));
            stored_previous
        } else {
            stored_previous
        }
    })
}