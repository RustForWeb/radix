use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes, A},
    path,
};

// use crate::primitives::{
//     accessible_icon, arrow, aspect_ratio, avatar, checkbox, collection, focus_scope, label, menu,
//     popper, portal, presence, progress, separator, slot, switch, toggle, visually_hidden,
// };
use crate::primitives::label;

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
                    // <li>
                    //     Accessible Icon

                    //     <ul class="ms-4">
                    //         <li><A href="/accessible-icon/styled">Styled</A></li>
                    //         <li><A href="/accessible-icon/chromatic">Chromatic</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Arrow

                    //     <ul class="ms-4">
                    //         <li><A href="/arrow/styled">Styled</A></li>
                    //         <li><A href="/arrow/custom-sizes">Custom Sizes</A></li>
                    //         <li><A href="/arrow/custom-arrow">Custom Arrow</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Aspect Ratio

                    //     <ul class="ms-4">
                    //         <li><A href="/aspect-ratio/styled">Styled</A></li>
                    //         <li><A href="/aspect-ratio/custom-ratios">Custom Ratios</A></li>
                    //         <li><A href="/aspect-ratio/chromatic">Chromatic</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Avatar

                    //     <ul class="ms-4">
                    //         <li><A href="/avatar/styled">Styled</A></li>
                    //         <li><A href="/avatar/chromatic">Chromatic</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Checkbox

                    //     <ul class="ms-4">
                    //         <li><A href="/checkbox/styled">Styled</A></li>
                    //         <li><A href="/checkbox/controlled">Controlled</A></li>
                    //         <li><A href="/checkbox/indeterminate">Indeterminate</A></li>
                    //         <li><A href="/checkbox/within-form">Within Form</A></li>
                    //         <li><A href="/checkbox/animated">Animated</A></li>
                    //         <li><A href="/checkbox/chromatic">Chromatic</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Collection

                    //     <ul class="ms-4">
                    //         <li><A href="/collection/basic">Basic</A></li>
                    //         <li><A href="/collection/with-element-in-between">With Element In Between</A></li>
                    //         <li><A href="/collection/with-wrapped-item">With Wrapped Item</A></li>
                    //         <li><A href="/collection/with-fragment">With Fragment</A></li>
                    //         <li><A href="/collection/dynamic-insertion">Dynamic Insertion</A></li>
                    //         <li><A href="/collection/with-changing-item">With Changing Item</A></li>
                    //         <li><A href="/collection/nested">Nested</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Focus Scope

                    //     <ul class="ms-4">
                    //         <li><A href="/focus-scope/basic">Basic</A></li>
                    //         <li><A href="/focus-scope/multiple">Multiple</A></li>
                    //     </ul>
                    // </li>
                    <li>
                        Label

                        <ul class="ms-4">
                            <li><A href="/label/styled">Styled</A></li>
                            <li><A href="/label/with-control">With Control</A></li>
                            <li><A href="/label/with-input-number">With Input Number</A></li>
                        </ul>
                    </li>
                    // <li>
                    //     Menu

                    //     <ul class="ms-4">
                    //         <li><A href="/menu/styled">Styled</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Popper

                    //     <ul class="ms-4">
                    //         <li><A href="/popper/styled">Styled</A></li>
                    //         <li><A href="/popper/with-custom-arrow">With Custom Arrow</A></li>
                    //         <li><A href="/popper/animated">Animated</A></li>
                    //         <li><A href="/popper/with-portal">With Portal</A></li>
                    //         <li><A href="/popper/with-update-position-strategy-always">With Update Position Strategy Always</A></li>
                    //         <li><A href="/popper/chromatic">Chromatic</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Portal

                    //     <ul class="ms-4">
                    //         <li><A href="/portal/base">Base</A></li>
                    //         <li><A href="/portal/custom-container">Custom Container</A></li>
                    //         <li><A href="/portal/chromatic">Chromatic</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Presence

                    //     <ul class="ms-4">
                    //         <li><A href="/presence/basic">Basic</A></li>
                    //         <li><A href="/presence/with-mount-animation">With Mount Animation</A></li>
                    //         <li><A href="/presence/with-unmount-animation">With Unmount Animation</A></li>
                    //         <li><A href="/presence/with-multiple-mount-animations">With Multiple Mount Animations</A></li>
                    //         <li><A href="/presence/with-open-and-close-animation">With Open and Close Animation</A></li>
                    //         <li><A href="/presence/with-multiple-open-and-close-animations">With Multiple Open and Close Animations</A></li>
                    //         <li><A href="/presence/with-deferred-mount-animation">With Deferred Mount Animation</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Progress

                    //     <ul class="ms-4">
                    //         <li><A href="/progress/styled">Styled</A></li>
                    //         <li><A href="/progress/chromatic">Chromatic</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Separator

                    //     <ul class="ms-4">
                    //         <li><A href="/separator/styled">Styled</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Slot

                    //     <ul class="ms-4">
                    //         <li><A href="/slot/without-slottable">Without Slottable</A></li>
                    //         <li><A href="/slot/with-slottable">With Slottable</A></li>
                    //         // TODO
                    //     </ul>
                    // </li>
                    // <li>
                    //     Switch

                    //     <ul class="ms-4">
                    //         <li><A href="/switch/styled">Styled</A></li>
                    //         <li><A href="/switch/controlled">Controlled</A></li>
                    //         <li><A href="/switch/within-form">Within Form</A></li>
                    //         <li><A href="/switch/chromatic">Chromatic</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Toggle

                    //     <ul class="ms-4">
                    //         <li><A href="/toggle/controlled">Controlled</A></li>
                    //         <li><A href="/toggle/chromatic">Chromatic</A></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Visually Hidden

                    //     <ul class="ms-4">
                    //         <li><A href="/visually-hidden/basic">Basic</A></li>
                    //     </ul>
                    // </li>
                </ul>
            </nav>
            <main class="ms-64 p-4">
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=path!("/") view=Index />

                    // <Route path="/accessible-icon/styled" view=accessible_icon::Styled />
                    // <Route path="/accessible-icon/chromatic" view=accessible_icon::Chromatic />

                    // <Route path="/arrow/styled" view=arrow::Styled />
                    // <Route path="/arrow/custom-sizes" view=arrow::CustomSizes />
                    // <Route path="/arrow/custom-arrow" view=arrow::CustomArrow />

                    // <Route path="/aspect-ratio/styled" view=aspect_ratio::Styled />
                    // <Route path="/aspect-ratio/custom-ratios" view=aspect_ratio::CustomRatios />
                    // <Route path="/aspect-ratio/chromatic" view=aspect_ratio::Chromatic />

                    // <Route path="/avatar/styled" view=avatar::Styled />
                    // <Route path="/avatar/chromatic" view=avatar::Chromatic />

                    // <Route path="/checkbox/styled" view=checkbox::Styled />
                    // <Route path="/checkbox/controlled" view=checkbox::Controlled />
                    // <Route path="/checkbox/indeterminate" view=checkbox::Indeterminate />
                    // <Route path="/checkbox/within-form" view=checkbox::WithinForm />
                    // <Route path="/checkbox/animated" view=checkbox::Animated />
                    // <Route path="/checkbox/chromatic" view=checkbox::Chromatic />

                    // <Route path="/collection/basic" view=collection::Basic />
                    // <Route path="/collection/with-element-in-between" view=collection::WithElementsInBetween />
                    // <Route path="/collection/with-wrapped-item" view=collection::WithWrappedItem />
                    // <Route path="/collection/with-fragment" view=collection::WithFragment />
                    // <Route path="/collection/dynamic-insertion" view=collection::DynamicInsertion />
                    // <Route path="/collection/with-changing-item" view=collection::WithChangingItem />
                    // <Route path="/collection/nested" view=collection::Nested />

                    // <Route path="/focus-scope/basic" view=focus_scope::Basic />
                    // <Route path="/focus-scope/multiple" view=focus_scope::Multiple />

                    <Route path=path!("/label/styled") view=label::Styled />
                    <Route path=path!("/label/with-control") view=label::WithControl />
                    <Route path=path!("/label/with-input-number") view=label::WithInputNumber />

                    // <Route path="/menu/styled" view=menu::Styled />

                    // <Route path="/popper/styled" view=popper::Styled />
                    // <Route path="/popper/with-custom-arrow" view=popper::WithCustomArrow />
                    // <Route path="/popper/animated" view=popper::Animated />
                    // <Route path="/popper/with-portal" view=popper::WithPortal />
                    // <Route path="/popper/with-update-position-strategy-always" view=popper::WithUpdatePositionStrategyAlways />
                    // <Route path="/popper/chromatic" view=popper::Chromatic />

                    // <Route path="/portal/base" view=portal::Base />
                    // <Route path="/portal/custom-container" view=portal::CustomContainer />
                    // <Route path="/portal/chromatic" view=portal::Chromatic />

                    // <Route path="/presence/basic" view=presence::Basic />
                    // <Route path="/presence/with-mount-animation" view=presence::WithMountAnimation />
                    // <Route path="/presence/with-unmount-animation" view=presence::WithUnmountAnimation />
                    // <Route path="/presence/with-multiple-mount-animations" view=presence::WithMultipleMountAnimations />
                    // <Route path="/presence/with-open-and-close-animation" view=presence::WithOpenAndCloseAnimation />
                    // <Route path="/presence/with-multiple-open-and-close-animations" view=presence::WithMultipleOpenAndCloseAnimations />
                    // <Route path="/presence/with-deferred-mount-animation" view=presence::WithDeferredMountAnimation />

                    // <Route path="/progress/styled" view=progress::Styled />
                    // <Route path="/progress/chromatic" view=progress::Chromatic />

                    // <Route path="/separator/styled" view=separator::Styled />

                    // <Route path="/slot/without-slottable" view=slot::WithoutSlottable />
                    // <Route path="/slot/with-slottable" view=slot::WithSlottable />

                    // <Route path="/switch/styled" view=switch::Styled />
                    // <Route path="/switch/controlled" view=switch::Controlled />
                    // <Route path="/switch/within-form" view=switch::WithinForm />
                    // <Route path="/switch/chromatic" view=switch::Chromatic />

                    // <Route path="/toggle/controlled" view=toggle::Controlled />
                    // <Route path="/toggle/chromatic" view=toggle::Chromatic />

                    // <Route path="/visually-hidden/basic" view=visually_hidden::Basic />
                </Routes>
            </main>
        </Router>
    }
}
