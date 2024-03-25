use axum::extract::FromRef;
use leptos::LeptosOptions;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
}
