mod fileserv;
mod state;

use axum::{
    body::Body, extract::State, http::Request, response::IntoResponse, routing::get, Router,
};
use dotenvy::dotenv;
use example_app::App;
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};

use crate::fileserv::file_and_error_handler;
use crate::state::AppState;

fn provide_context_from_app_state(_app_state: AppState) {
    // provide_context(app_state.database.clone());
}

async fn server_fns_handler(
    State(app_state): State<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context_from_app_state(app_state.clone());
        },
        req,
    )
    .await
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState { leptos_options };
    let app_state_context = app_state.clone();

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fns_handler).post(server_fns_handler),
        )
        .leptos_routes_with_context(
            &app_state,
            routes,
            move || {
                provide_context_from_app_state(app_state_context.clone());
            },
            App,
        )
        .fallback(file_and_error_handler)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("Listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
