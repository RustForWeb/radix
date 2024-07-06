use leptos::*;
use leptos_router::{Route, Router, Routes, A};

use crate::primitives::{
    arrow, avatar, collection, focus_scope, label, menu, popper, slot, visually_hidden,
};

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
                        Arrow

                        <ul class="ms-4">
                            <li><A href="/arrow/styled">Styled</A></li>
                            <li><A href="/arrow/custom-sizes">Custom Sizes</A></li>
                            <li><A href="/arrow/custom-arrow">Custom Arrow</A></li>
                        </ul>
                    </li>
                    <li>
                        Avatar

                        <ul class="ms-4">
                            <li><A href="/avatar/styled">Styled</A></li>
                            <li><A href="/avatar/chromatic">Chromatic</A></li>
                        </ul>
                    </li>
                    <li>
                        Collection

                        <ul class="ms-4">
                            <li><A href="/collection/basic">Basic</A></li>
                            <li><A href="/collection/with-element-in-between">With Element In Between</A></li>
                            <li><A href="/collection/with-wrapped-item">With Wrapped Item</A></li>
                            <li><A href="/collection/with-fragment">With Fragment</A></li>
                            <li><A href="/collection/dynamic-insertion">Dynamic Insertion</A></li>
                            <li><A href="/collection/with-changing-item">With Changing Item</A></li>
                            <li><A href="/collection/nested">Nested</A></li>
                        </ul>
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
                    <li>
                        Slot

                        <ul class="ms-4">
                            <li><A href="/slot/without-slottable">Without Slottable</A></li>
                            <li><A href="/slot/with-slottable">With Slottable</A></li>
                            // TODO
                        </ul>
                    </li>
                    <li>
                        Visually Hidden

                        <ul class="ms-4">
                            <li><A href="/visually-hidden/basic">Basic</A></li>
                        </ul>
                    </li>
                </ul>
            </nav>
            <main class="ms-64 p-4">
                <Routes>
                    <Route path="/" view=Index />

                    <Route path="/arrow/styled" view=arrow::Styled />
                    <Route path="/arrow/custom-sizes" view=arrow::CustomSizes />
                    <Route path="/arrow/custom-arrow" view=arrow::CustomArrow />

                    <Route path="/avatar/styled" view=avatar::Styled />
                    <Route path="/avatar/chromatic" view=avatar::Chromatic />

                    <Route path="/collection/basic" view=collection::Basic />
                    <Route path="/collection/with-element-in-between" view=collection::WithElementsInBetween />
                    <Route path="/collection/with-wrapped-item" view=collection::WithWrappedItem />
                    <Route path="/collection/with-fragment" view=collection::WithFragment />
                    <Route path="/collection/dynamic-insertion" view=collection::DynamicInsertion />
                    <Route path="/collection/with-changing-item" view=collection::WithChangingItem />
                    <Route path="/collection/nested" view=collection::Nested />

                    <Route path="/focus-scope/basic" view=focus_scope::Basic />
                    <Route path="/focus-scope/multiple" view=focus_scope::Multiple />

                    <Route path="/label/styled" view=label::Styled />
                    <Route path="/label/with-control" view=label::WithControl />
                    <Route path="/label/with-input-number" view=label::WithInputNumber />

                    <Route path="/menu/styled" view=menu::Styled />

                    <Route path="/popper/styled" view=popper::Styled />

                    <Route path="/slot/without-slottable" view=slot::WithoutSlottable />
                    <Route path="/slot/with-slottable" view=slot::WithSlottable />

                    <Route path="/visually-hidden/basic" view=visually_hidden::Basic />
                </Routes>
            </main>
        </Router>
    }
}
