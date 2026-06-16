use std::sync::Arc;
use axum::{Extension, response::IntoResponse};
use axum_inertia::Inertia;
use serde_json::json;

use crate::app::AppState;

/// GET /about — render About page via Inertia.
pub async fn index(
    inertia: Inertia,
    Extension(_state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    inertia.render("About", json!({
        "title": "About Laju Rust",
        "stack": [
            "Axum",
            "Sailfish",
            "RocksDB",
            "Inertia.js v3",
            "Svelte",
            "Vite 8",
            "Tailwind CSS v4"
        ]
    }))
}
