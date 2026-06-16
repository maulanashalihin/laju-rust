use std::sync::Arc;
use axum::{Extension, response::IntoResponse};
use axum_inertia::Inertia;
use serde_json::json;

use crate::app::AppState;
use crate::services::example::GreetService;

/// GET / — render Home page via Inertia.
pub async fn index(
    inertia: Inertia,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let svc = GreetService::new(state.db.clone());
    let greeting = svc.greet("Laju Rust");
    tracing::debug!("{}", greeting);

    inertia.render("Home", json!({
        "title": "Welcome to Laju Rust",
        "description": "A modern full-stack boilerplate with Axum, Inertia.js, Svelte, and Tailwind CSS",
    }))
}
