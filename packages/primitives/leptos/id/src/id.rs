use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::*;

static COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn use_id(deterministic_id: Option<String>) -> ReadSignal<String> {
    let (id, _) = create_signal(
        deterministic_id
            .unwrap_or_else(|| format!("radix-{}", COUNT.fetch_add(1, Ordering::Relaxed))),
    );

    id
}
