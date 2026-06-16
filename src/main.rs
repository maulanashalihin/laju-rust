mod app;
mod config;
mod db;
mod handlers;
mod inertia;
mod middleware;
mod models;
mod routes;
mod services;

use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("laju_rust=debug,tower_http=debug")
        .init();

    let cfg = config::AppConfig::from_env();
    let inertia_cfg = inertia::build(&cfg);

    let app = app::build(inertia_cfg);
    let addr = &cfg.server_addr;

    tracing::info!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}