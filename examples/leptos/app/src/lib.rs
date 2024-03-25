// TODO: remove once this Leptos bug has been fixed
#![allow(clippy::empty_docs)]

pub mod components;
pub mod error_template;
pub mod pages;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::layout::Layout;
use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::home::HomePage;
use crate::pages::popper::PopperPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/example.css" />

        <Title text="Leptos Radix Example" />

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Layout>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="/popper" view=PopperPage />
                </Routes>
            </Layout>
        </Router>
    }
}
