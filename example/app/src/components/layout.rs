use leptos::*;

use super::navigation::topbar::Topbar;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <header class="fixed inset-x-0">
            <Topbar />
        </header>

        <main class="container h-screen w-screen pt-16">
            {children()}
        </main>

        <footer />
    }
}
