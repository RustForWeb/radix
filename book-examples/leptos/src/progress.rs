use std::time::Duration;

use leptos::*;
use radix_leptos_progress::*;

#[component]
pub fn ProgressDemo() -> impl IntoView {
    let (progress, set_progress) = create_signal(13.0);

    let handle = set_timeout_with_handle(
        move || {
            set_progress.set(66.0);
        },
        Duration::from_millis(500),
    )
    .expect("Timeout should be started.");

    on_cleanup(move || handle.clear());

    view! {
        <Progress
            attr:class="relative overflow-hidden bg-blackA6 rounded-full w-[300px] h-[25px]"
            // Fix overflow clipping in Safari
            // https://gist.github.com/domske/b66047671c780a238b51c51ffde8d3a0
            attr:style="transform: translateZ(0)"
            value=progress
        >
            <ProgressIndicator
                attr:class="bg-white w-full h-full transition-transform duration-[660ms] ease-[cubic-bezier(0.65, 0, 0.35, 1)]"
                attr:style=move || format!("transform: translateX(-{}%)", 100.0 - progress.get())
            />
        </Progress>
    }
}
