use std::sync::Arc;
use axum::{Extension, Router, middleware};
use tower_http::services::ServeDir;

use crate::config::AppConfig;
use crate::routes;

#[derive(Clone)]
pub enum DbPool {
    RocksDb(Arc<rocksdb::DB>),
    Sqlite(sqlx::SqlitePool),
}

/// Shared application state — di-inject ke handlers via Extension.
pub struct AppState {
    pub db: DbPool,
    pub config: AppConfig,
}

/// Build Axum Router dengan semua routes, middleware, dan state.
pub fn build(inertia_config: axum_inertia::InertiaConfig) -> Router {
    let config = AppConfig::from_env();

    let db = match config.db_backend.as_str() {
        "sqlite" => {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            let pool = rt.block_on(async {
                crate::db::sqlite::init(&format!("sqlite:{}", config.db_path.replace("rocksdb", "sqlite")))
                    .await
                    .expect("Failed to initialize SQLite")
            });
            DbPool::Sqlite(pool)
        }
        _ => {
            let rocks = Arc::new(crate::db::rocksdb::init(&config.db_path)
                .expect("Failed to initialize RocksDB"));
            DbPool::RocksDb(rocks)
        }
    };

    let app_state = Arc::new(AppState { db, config: config.clone() });

    routes::register(Router::new())
        .nest_service("/assets", ServeDir::new("dist/assets"))
        .nest_service("/public", ServeDir::new("public"))
        .layer(middleware::from_fn(crate::middleware::auth::resolve_user))
        .layer(Extension(app_state))
        .with_state(inertia_config)
}
