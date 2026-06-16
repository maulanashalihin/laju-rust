use std::sync::Arc;
use axum::{Extension, Router, middleware};
use tower_http::services::ServeDir;

use crate::config::AppConfig;
use crate::routes;

/// Shared application state — di-inject ke handlers via Extension.
pub struct AppState {
    pub db: Arc<rocksdb::DB>,
    pub config: AppConfig,
}

/// Build Axum Router dengan semua routes, middleware, dan state.
/// `.with_state()` dipanggil di akhir agar type inference bekerja.
pub fn build(inertia_config: axum_inertia::InertiaConfig) -> Router {
    let config = AppConfig::from_env();
    let db = Arc::new(crate::db::rocksdb::init(&config.db_path)
        .expect("Failed to initialize RocksDB"));

    let app_state = Arc::new(AppState { db, config });

    routes::register(Router::new())
        .nest_service("/assets", ServeDir::new("dist/assets"))
        .layer(middleware::from_fn(crate::middleware::auth::resolve_user))
        .layer(Extension(app_state))
        .layer(middleware::from_fn(crate::middleware::request_id::middleware))
        .with_state(inertia_config)
}
