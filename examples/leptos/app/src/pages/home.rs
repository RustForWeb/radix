use leptos::*;
use leptos_router::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1 class="text-xl pb-3">Leptos Radix Example</h1>

        <ul class="list-disc list-inside">
            <li><A class="underline hover:text-primary" href="/popper">Popper</A></li>
        </ul>
    }
}
