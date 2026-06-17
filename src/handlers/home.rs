use sailfish::TemplateSimple;
use axum::{Extension, response::{Html, IntoResponse}};
use std::sync::Arc;
use crate::app::AppState;

#[derive(TemplateSimple)]
#[template(path = "home.stpl")]
struct HomeTemplate {
    title: String,
    description: String,
    dev_mode: bool,
    asset_url: String,
}

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let asset_url = if state.config.dev_mode {
        state.config.vite_dev_url.clone()
    } else {
        String::new()
    };

    let tpl = HomeTemplate {
        title: "Welcome to Laju Rust".into(),
        description: "A modern full-stack boilerplate with Axum, Inertia.js, Svelte, and Tailwind CSS".into(),
        dev_mode: state.config.dev_mode,
        asset_url,
    };

    Html(tpl.render_once().unwrap_or_else(|e| format!("Template error: {}", e)))
}
