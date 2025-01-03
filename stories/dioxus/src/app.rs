use dioxus::prelude::*;

use crate::use_node_ref::UseNodeRef;

#[derive(Clone, Routable)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        #[route("/")]
        Index {},
}

#[component]
fn Layout() -> Element {
    rsx! {
        nav {
            class: "bg-slate-200 p-4 fixed top-0 bottom-0 start-0 w-64 overflow-y-auto",
            ul {
                li {
                    Link {
                        to: Route::Index {},
                        "Index"
                    }
                }
            }
        }
        main {
            class: "ms-64 p-4",
            Outlet::<Route> {}
        }
    }
}

#[component]
fn Index() -> Element {
    rsx! {
        h1 { "Radix Dioxus Stories" }
        UseNodeRef {}
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
