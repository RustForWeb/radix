use std::rc::Rc;

use yew::prelude::*;

#[hook]
pub fn use_previous<T>(value: T) -> Rc<T>
where
    T: Clone + PartialEq + 'static,
{
    let current = use_state_eq(|| value.clone());
    let previous = use_state_eq(|| value.clone());

    use_memo(value, move |value| {
        if *current != *value {
            previous.set((*current).clone());
            current.set(value.clone());
        }

        (*previous).clone()
    })
}
