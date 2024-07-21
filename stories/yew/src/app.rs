use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Index,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! { <Index /> },
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
                </ul>
            </nav>
            <main class="ms-64 p-4">
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}
