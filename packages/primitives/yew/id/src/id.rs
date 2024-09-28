use std::sync::atomic::{AtomicUsize, Ordering};

use yew::prelude::*;

static COUNT: AtomicUsize = AtomicUsize::new(0);

#[hook]
pub fn use_id(deterministic_id: Option<String>) -> String {
    let id = use_state_eq(|| {
        deterministic_id
            .unwrap_or_else(|| format!("radix-{}", COUNT.fetch_add(1, Ordering::Relaxed)))
    });

    (*id).clone()
}
