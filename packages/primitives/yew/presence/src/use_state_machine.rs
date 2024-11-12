use std::{fmt::Debug, hash::Hash, marker::PhantomData, ops::Deref, rc::Rc};

use yew::prelude::*;

pub trait MachineState<E> {
    fn next(&self, event: E) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq)]
pub struct Machine<S: MachineState<E>, E> {
    state: S,
    phantom: PhantomData<E>,
}

impl<S: MachineState<E>, E> Machine<S, E> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            phantom: PhantomData,
        }
    }
}

impl<S, E> Deref for Machine<S, E>
where
    S: MachineState<E>,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<S, E> Reducible for Machine<S, E>
where
    S: MachineState<E> + Clone + Debug + Eq + Hash + 'static,
    E: Clone + Debug + Eq + Hash + 'static,
{
    type Action = E;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        if let Some(next_state) = self.state.next(action) {
            Rc::new(Machine::new(next_state))
        } else {
            self
        }
    }
}

#[hook]
pub fn use_state_machine<S, E>(initial_state: S) -> UseReducerHandle<Machine<S, E>>
where
    S: MachineState<E> + Clone + Debug + Eq + Hash + 'static,
    E: Clone + Debug + Eq + Hash + 'static,
{
    use_reducer(|| Machine::new(initial_state))

    // (
    //     state,
    //     Callback::new(move |event| {
    //         let current_state = state.get_untracked();
    //         let next_state = machine
    //             .get(&current_state)
    //             .and_then(|events| events.get(&event));

    //         if let Some(next_state) = next_state {
    //             set_state.set(next_state.clone());
    //         }
    //     }),
    // )
}
