use yew::prelude::*;

pub struct UseControllableStateParams<T: PartialEq + 'static> {
    pub prop: Option<T>,
    pub default_prop: Option<T>,
    pub on_change: Option<Callback<Option<T>>>,
}

#[hook]
pub fn use_controllable_state<T>(
    params: UseControllableStateParams<T>,
) -> (Option<T>, Callback<Option<T>>)
where
    T: Clone + PartialEq + 'static,
{
    let UseControllableStateParams {
        prop,
        default_prop,
        on_change,
    } = params;

    let uncontrolled_prop = use_uncontrolled_state(UseUncontrollableStateParams {
        default_prop,
        on_change: on_change.clone(),
    });
    let is_controlled = prop.is_some();
    let value = match is_controlled {
        true => prop.clone(),
        false => (*uncontrolled_prop).clone(),
    };

    let set_value = Callback::from(move |next_value| {
        if is_controlled {
            if next_value != prop {
                if let Some(on_change) = &on_change {
                    on_change.emit(next_value);
                }
            }
        } else {
            uncontrolled_prop.set(next_value);
        }
    });

    (value, set_value)
}

pub struct UseUncontrollableStateParams<T: PartialEq + 'static> {
    pub default_prop: Option<T>,
    pub on_change: Option<Callback<Option<T>>>,
}

#[hook]
fn use_uncontrolled_state<T>(params: UseUncontrollableStateParams<T>) -> UseStateHandle<Option<T>>
where
    T: Clone + PartialEq + 'static,
{
    let UseUncontrollableStateParams {
        default_prop,
        on_change,
    } = params;

    let value = use_state_eq(|| default_prop);
    let prev_value = use_state_eq(|| (*value).clone());

    use_effect_with((value.clone(), prev_value), |(value, prev_value)| {
        let value = (**value).clone();
        if **prev_value != value {
            if let Some(on_change) = on_change {
                on_change.emit(value.clone());
                prev_value.set(value);
            }
        }
    });

    value
}
