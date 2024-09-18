use yew::prelude::*;
use yew_router::prelude::*;

use crate::primitives::{arrow, label, popper, slot};

#[derive(Clone, Copy, Debug, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Index,

    #[at("/arrow/styled")]
    ArrowStyled,
    #[at("/arrow/custom-sizes")]
    ArrowCustomSizes,
    #[at("/arrow/custom-arrow")]
    ArrowCustomArrow,

    #[at("/label/styled")]
    LabelStyled,
    #[at("/label/with-control")]
    LabelWithControl,
    #[at("/label/with-input-number")]
    LabelWithInputNumber,

    #[at("/popper/styled")]
    PopperStyled,
    #[at("/popper/with-custom-arrow")]
    PopperWithCustomArrow,
    #[at("/popper/animated")]
    PopperAnimated,
    #[at("/popper/with-portal")]
    PopperWithPortal,
    #[at("/popper/with-update-position-strategy-always")]
    PopperWithUpdatePositionStrategyAlways,
    #[at("/popper/chromatic")]
    PopperChromatic,

    #[at("/slot/without-slottable")]
    SlotWithoutSlottable,
    #[at("/slot/with-slottable")]
    SlotWithSlottable,
    #[at("/slot/with-composed-events")]
    SlotWithComposedEvents,
    #[at("/slot/button-as-link")]
    SlotButtonAsLink,
    #[at("/slot/chromatic")]
    SlotChromatic,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! { <Index /> },

        Route::ArrowStyled => html! { <arrow::Styled /> },
        Route::ArrowCustomSizes => html! { <arrow::CustomSizes /> },
        Route::ArrowCustomArrow => html! { <arrow::CustomArrow /> },

        Route::LabelStyled => html! { <label::Styled /> },
        Route::LabelWithControl => html! { <label::WithControl /> },
        Route::LabelWithInputNumber => html! { <label::WithInputNumber /> },

        Route::PopperStyled => html! { <popper::Styled /> },
        Route::PopperWithCustomArrow => html! { <popper::WithCustomArrow /> },
        Route::PopperAnimated => html! { <popper::Animated /> },
        Route::PopperWithPortal => html! { <popper::WithPortal /> },
        Route::PopperWithUpdatePositionStrategyAlways => {
            html! { <popper::WithUpdatePositionStrategyAlways /> }
        }
        Route::PopperChromatic => html! { <popper::Chromatic /> },

        Route::SlotWithoutSlottable => html! { <slot::WithoutSlottable /> },
        Route::SlotWithSlottable => html! { <slot::WithSlottable /> },
        Route::SlotWithComposedEvents => html! { <slot::WithComposedEvents /> },
        Route::SlotButtonAsLink => html! { <slot::ButtonAsLink /> },
        Route::SlotChromatic => html! { <slot::Chromatic /> },
    }
}

#[function_component]
fn Index() -> Html {
    html! {
        <h1>{ "Radix Yew Stories" }</h1>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <nav class="bg-slate-200 p-4 fixed top-0 bottom-0 start-0 w-64 overflow-y-auto">
                <ul>
                    <li>
                        <Link<Route> to={Route::Index}>{ "Index" }</Link<Route>>
                    </li>
                    <li>
                        {"Arrow"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::ArrowStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::ArrowCustomSizes}>{"Custom Sizes"}</Link<Route>></li>
                            <li><Link<Route> to={Route::ArrowCustomArrow}>{"Custom Arrow"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Label"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::LabelStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::LabelWithControl}>{"With Control"}</Link<Route>></li>
                            <li><Link<Route> to={Route::LabelWithInputNumber}>{"With Input Number"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Popper"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::PopperStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PopperWithCustomArrow}>{"With Custom Arrow"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PopperAnimated}>{"Animated"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PopperWithPortal}>{"With Portal"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PopperWithUpdatePositionStrategyAlways}>{"With Update Position Strategy Always"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PopperChromatic}>{"Chromatic"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Slot"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::SlotWithoutSlottable}>{"Without Slottable"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SlotWithSlottable}>{"With Slottable"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SlotWithComposedEvents}>{"With Composed Events"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SlotButtonAsLink}>{"Button As Link"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SlotChromatic}>{"Chromatic"}</Link<Route>></li>
                        </ul>
                    </li>
                </ul>
            </nav>
            <main class="ms-64 p-4">
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}
