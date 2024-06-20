use leptos::*;
use leptos_router::{Route, Router, Routes, A};

use crate::primitives::{focus_scope, label, menu, popper};

#[component]
fn Index() -> impl IntoView {
    view! {
        <h1>Radix Leptos Stories</h1>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav class="bg-slate-200 p-4 fixed top-0 bottom-0 start-0 w-64 overflow-y-auto">
                <ul>
                    <li>
                        <A href="/">Index</A>
                    </li>
                    <li>
                        Focus Scope

                        <ul class="ms-4">
                            <li><A href="/focus-scope/basic">Basic</A></li>
                            <li><A href="/focus-scope/multiple">Multiple</A></li>
                        </ul>
                    </li>
                    <li>
                        Label

                        <ul class="ms-4">
                            <li><A href="/label/styled">Styled</A></li>
                            <li><A href="/label/with-control">With Control</A></li>
                            <li><A href="/label/with-input-number">With Input Number</A></li>
                        </ul>
                    </li>
                    <li>
                        Menu

                        <ul class="ms-4">
                            <li><A href="/menu/styled">Styled</A></li>
                        </ul>
                    </li>
                    <li>
                        Popper

                        <ul class="ms-4">
                            <li><A href="/popper/styled">Styled</A></li>
                        </ul>
                    </li>
                </ul>
            </nav>
            <main class="ms-64 p-4">
                <Routes>
                    <Route path="/" view=Index />

                    <Route path="/focus-scope/basic" view=focus_scope::Basic />
                    <Route path="/focus-scope/multiple" view=focus_scope::Multiple />

                    <Route path="/label/styled" view=label::Styled />
                    <Route path="/label/with-control" view=label::WithControl />
                    <Route path="/label/with-input-number" view=label::WithInputNumber />

                    <Route path="/menu/styled" view=menu::Styled />

                    <Route path="/popper/styled" view=popper::Styled />
                </Routes>
            </main>
        </Router>
    }
}
