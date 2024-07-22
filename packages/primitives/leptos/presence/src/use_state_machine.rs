use std::fmt::Debug;
use std::{collections::HashMap, hash::Hash};

use leptos::{create_signal, Callback, ReadSignal, SignalGetUntracked, SignalSet};

type Machine<S, E> = HashMap<S, HashMap<E, S>>;

pub fn use_state_machine<S: Clone + Debug + Eq + Hash, E: Clone + Debug + Eq + Hash + 'static>(
    initial_state: S,
    machine: Machine<S, E>,
) -> (ReadSignal<S>, Callback<E>) {
    let (state, set_state) = create_signal(initial_state);

    (
        state,
        Callback::new(move |event| {
            let current_state = state.get_untracked();
            let next_state = machine
                .get(&current_state)
                .and_then(|events| events.get(&event));

            if let Some(next_state) = next_state {
                set_state.set(next_state.clone());
            }
        }),
    )
}
