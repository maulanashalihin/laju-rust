use axum::response::IntoResponse;
use axum_inertia::Inertia;
use serde_json::json;

/// GET / — render Home page via Inertia.
pub async fn index(inertia: Inertia) -> impl IntoResponse {
    inertia.render("Home", json!({
        "title": "Welcome to Laju Rust",
        "description": "A modern full-stack boilerplate with Axum, Inertia.js, Svelte, and Tailwind CSS",
    }))
}
