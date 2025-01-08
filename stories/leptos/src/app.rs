use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes, ToHref, A},
    path,
};

use crate::primitives::{accessible_icon, arrow, aspect_ratio, label, visually_hidden};

#[component]
fn NavLink<H>(href: H, children: Children) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    // TODO: add class when active
    view! {
        <A href=href attr:class="text-inherit decoration-inherit no-underline">
            {children()}
        </A>
    }
}

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
            <nav class="bg-slate-200 p-4 fixed top-0 bottom-0 start-0 w-64 box-border overflow-y-auto leading-normal">
                <ul class="list-none m-0 p-0">
                    <li>
                        <NavLink href="/">Index</NavLink>
                    </li>
                    <li>
                        Accessible Icon

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/accessible-icon/styled">Styled</NavLink></li>
                            <li><NavLink href="/accessible-icon/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Arrow

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/arrow/styled">Styled</NavLink></li>
                            <li><NavLink href="/arrow/custom-sizes">Custom Sizes</NavLink></li>
                            <li><NavLink href="/arrow/custom-arrow">Custom Arrow</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Aspect Ratio

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/aspect-ratio/styled">Styled</NavLink></li>
                            <li><NavLink href="/aspect-ratio/custom-ratios">Custom Ratios</NavLink></li>
                            <li><NavLink href="/aspect-ratio/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    // <li>
                    //     Avatar

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/avatar/styled">Styled</NavLink></li>
                    //         <li><NavLink href="/avatar/chromatic">Chromatic</NavLink></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Checkbox

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/checkbox/styled">Styled</NavLink></li>
                    //         <li><NavLink href="/checkbox/controlled">Controlled</NavLink></li>
                    //         <li><NavLink href="/checkbox/indeterminate">Indeterminate</NavLink></li>
                    //         <li><NavLink href="/checkbox/within-form">Within Form</NavLink></li>
                    //         <li><NavLink href="/checkbox/animated">Animated</NavLink></li>
                    //         <li><NavLink href="/checkbox/chromatic">Chromatic</NavLink></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Collection

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/collection/basic">Basic</NavLink></li>
                    //         <li><NavLink href="/collection/with-element-in-between">With Element In Between</NavLink></li>
                    //         <li><NavLink href="/collection/with-wrapped-item">With Wrapped Item</NavLink></li>
                    //         <li><NavLink href="/collection/with-fragment">With Fragment</NavLink></li>
                    //         <li><NavLink href="/collection/dynamic-insertion">Dynamic Insertion</NavLink></li>
                    //         <li><NavLink href="/collection/with-changing-item">With Changing Item</NavLink></li>
                    //         <li><NavLink href="/collection/nested">Nested</NavLink></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Focus Scope

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/focus-scope/basic">Basic</NavLink></li>
                    //         <li><NavLink href="/focus-scope/multiple">Multiple</NavLink></li>
                    //     </ul>
                    // </li>
                    <li>
                        Label

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/label/basic">Basic</NavLink></li>
                            <li><NavLink href="/label/with-control">With Control</NavLink></li>
                            <li><NavLink href="/label/with-inputs">With Inputs</NavLink></li>
                            <li><NavLink href="/label/with-events">With Events</NavLink></li>
                        </ul>
                    </li>
                    // <li>
                    //     Menu

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/menu/styled">Styled</NavLink></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Popper

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/popper/styled">Styled</NavLink></li>
                    //         <li><NavLink href="/popper/with-custom-arrow">With Custom Arrow</NavLink></li>
                    //         <li><NavLink href="/popper/animated">Animated</NavLink></li>
                    //         <li><NavLink href="/popper/with-portal">With Portal</NavLink></li>
                    //         <li><NavLink href="/popper/with-update-position-strategy-always">With Update Position Strategy Always</NavLink></li>
                    //         <li><NavLink href="/popper/chromatic">Chromatic</NavLink></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Portal

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/portal/base">Base</NavLink></li>
                    //         <li><NavLink href="/portal/custom-container">Custom Container</NavLink></li>
                    //         <li><NavLink href="/portal/chromatic">Chromatic</NavLink></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Presence

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/presence/basic">Basic</NavLink></li>
                    //         <li><NavLink href="/presence/with-mount-animation">With Mount Animation</NavLink></li>
                    //         <li><NavLink href="/presence/with-unmount-animation">With Unmount Animation</NavLink></li>
                    //         <li><NavLink href="/presence/with-multiple-mount-animations">With Multiple Mount Animations</NavLink></li>
                    //         <li><NavLink href="/presence/with-open-and-close-animation">With Open and Close Animation</NavLink></li>
                    //         <li><NavLink href="/presence/with-multiple-open-and-close-animations">With Multiple Open and Close Animations</NavLink></li>
                    //         <li><NavLink href="/presence/with-deferred-mount-animation">With Deferred Mount Animation</NavLink></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Progress

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/progress/styled">Styled</NavLink></li>
                    //         <li><NavLink href="/progress/chromatic">Chromatic</NavLink></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Separator

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/separator/styled">Styled</NavLink></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Slot

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/slot/without-slottable">Without Slottable</NavLink></li>
                    //         <li><NavLink href="/slot/with-slottable">With Slottable</NavLink></li>
                    //         // TODO
                    //     </ul>
                    // </li>
                    // <li>
                    //     Switch

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/switch/styled">Styled</NavLink></li>
                    //         <li><NavLink href="/switch/controlled">Controlled</NavLink></li>
                    //         <li><NavLink href="/switch/within-form">Within Form</NavLink></li>
                    //         <li><NavLink href="/switch/chromatic">Chromatic</NavLink></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Toggle

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/toggle/controlled">Controlled</NavLink></li>
                    //         <li><NavLink href="/toggle/chromatic">Chromatic</NavLink></li>
                    //     </ul>
                    // </li>
                    <li>
                        Visually Hidden

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/visually-hidden/basic">Basic</NavLink></li>
                        </ul>
                    </li>
                </ul>
            </nav>
            <main class="ms-64 p-4">
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=path!("/") view=Index />

                    <Route path=path!("/accessible-icon/styled") view=accessible_icon::Styled />
                    <Route path=path!("/accessible-icon/chromatic") view=accessible_icon::Chromatic />

                    <Route path=path!("/arrow/styled") view=arrow::Styled />
                    <Route path=path!("/arrow/custom-sizes") view=arrow::CustomSizes />
                    <Route path=path!("/arrow/custom-arrow") view=arrow::CustomArrow />

                    <Route path=path!("/aspect-ratio/styled") view=aspect_ratio::Styled />
                    <Route path=path!("/aspect-ratio/custom-ratios") view=aspect_ratio::CustomRatios />
                    <Route path=path!("/aspect-ratio/chromatic") view=aspect_ratio::Chromatic />

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

                    <Route path=path!("/label/basic") view=label::Basic />
                    <Route path=path!("/label/with-control") view=label::WithControl />
                    <Route path=path!("/label/with-inputs") view=label::WithInputs />
                    <Route path=path!("/label/with-events") view=label::WithEvents />

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

                    <Route path=path!("/visually-hidden/basic") view=visually_hidden::Basic />
                </Routes>
            </main>
        </Router>
    }
}
