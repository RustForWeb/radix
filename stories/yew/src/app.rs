use yew::prelude::*;
use yew_router::prelude::*;

use crate::primitives::label;

#[derive(Clone, Copy, Debug, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Index,

    #[at("/label/styled")]
    LabelStyled,
    #[at("/label/with-control")]
    LabelWithControl,
    #[at("/label/with-input-number")]
    LabelWithInputNumber,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! { <Index /> },
        Route::LabelStyled => html! { <label::Styled /> },
        Route::LabelWithControl => html! { <label::WithControl /> },
        Route::LabelWithInputNumber => html! { <label::WithInputNumber /> },
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
                        {"Label"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::LabelStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::LabelWithControl}>{"With Control"}</Link<Route>></li>
                            <li><Link<Route> to={Route::LabelWithInputNumber}>{"With Input Number"}</Link<Route>></li>
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
