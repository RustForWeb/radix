use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::primitives::slot::{
    ButtonAsLink as SlotButtonAsLink, Chromatic as SlotChromatic,
    WithComposedEvents as SlotWithComposedEvents, WithSlottable as SlotWithSlottable,
    WithoutSlottable as SlotWithoutSlottable,
};
use crate::use_node_ref::UseNodeRef;

#[derive(Clone, Routable)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        #[route("/")]
        Index {},

        #[route("/slot/without-slottable")]
        SlotWithoutSlottable {},
        #[route("/slot/with-slottable")]
        SlotWithSlottable {},
        #[route("/slot/with-composed-events")]
        SlotWithComposedEvents {},
        #[route("/slot/with-composed-events")]
        SlotButtonAsLink {},
        #[route("/slot/with-composed-events")]
        SlotChromatic {},
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
                li {
                    "Slot"


                    ul {
                        class: "ms-4",
                        li {
                            Link {
                                to: Route::SlotWithoutSlottable {},
                                "Without Slottable"
                            }
                        }
                        li {
                            Link {
                                to: Route::SlotWithSlottable {},
                                "With Slottable"
                            }
                        }
                        li {
                            Link {
                                to: Route::SlotWithComposedEvents {},
                                "With Composed Events"
                            }
                        }
                        li {
                            Link {
                                to: Route::SlotButtonAsLink {},
                                "Button As Link"
                            }
                        }
                        li {
                            Link {
                                to: Route::SlotChromatic {},
                                "Chromatic"
                            }
                        }
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
        Router::<Route> {
            config: || RouterConfig::default().history(WebHistory::default())
        }
    }
}
