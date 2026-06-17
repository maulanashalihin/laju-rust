use std::sync::Arc;
use std::time::Instant;
use axum::{Extension, Router, middleware};
use axum::body::Body;
use axum::http::Request;
use tower::ServiceExt;
use laju_rust::app::{AppState, DbPool};
use laju_rust::config::AppConfig;
use laju_rust::routes;

struct RockHandle {
    db: Arc<rocksdb::DB>,
    _dir: tempfile::TempDir,
}

fn build_router(db: DbPool) -> Router {
    let mut config = AppConfig::from_env();
    config.dev_mode = true;

    let state = Arc::new(AppState { db, config: config.clone() });

    let layout = move |_: String| -> String { String::new() };
    let inertia_cfg = axum_inertia::InertiaConfig::new(None::<String>, Box::new(layout));

    routes::register(Router::new())
        .layer(middleware::from_fn(laju_rust::middleware::auth::resolve_user))
        .layer(Extension(state))
        .with_state(inertia_cfg)
}

fn rocksdb_pool() -> (DbPool, RockHandle) {
    let dir = tempfile::tempdir().unwrap();
    let mut opts = rocksdb::Options::default();
    opts.create_if_missing(true);
    let db = Arc::new(rocksdb::DB::open(&opts, dir.path()).unwrap());
    let handle = RockHandle { db: db.clone(), _dir: dir };
    (DbPool::RocksDb(db), handle)
}

async fn sqlite_pool() -> DbPool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY, name TEXT NOT NULL, email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL, role TEXT NOT NULL DEFAULT 'admin'
        )"
    ).execute(&pool).await.unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY, user_id TEXT NOT NULL REFERENCES users(id),
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )"
    ).execute(&pool).await.unwrap();
    DbPool::Sqlite(pool)
}

fn print_row(label: &str, count: usize, trips: &[std::time::Duration]) {
    let total: std::time::Duration = trips.iter().sum();
    let ops = if total.as_secs_f64() > 0.0 {
        (count as f64 / total.as_secs_f64()) as u64
    } else { 0 };

    let mut sorted = trips.to_vec();
    sorted.sort();
    let p50 = sorted[sorted.len() / 2];
    let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
    let p99 = sorted[(sorted.len() as f64 * 0.99) as usize];

    let f = |n: u64| -> String {
        if n >= 1_000_000 { format!("{:.1}M", n as f64 / 1_000_000.0) }
        else if n >= 1_000 { format!("{:.0}K", n as f64 / 1_000.0) }
        else { n.to_string() }
    };

    println!("  {:<25} {:>8}   P50:{:>6}ms P95:{:>6}ms P99:{:>6}ms",
        label, f(ops), p50.as_millis(), p95.as_millis(), p99.as_millis());
}

#[tokio::main]
async fn main() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║     Laju Rust — HTTP Endpoint Benchmark    ║");
    println!("╚══════════════════════════════════════════════╝\n");

    let n = 200;

    for &(label, is_sqlite) in &[("RocksDB", false), ("SQLite", true)] {
        println!("── {} ────────────────────────────────\n", label);

        let (pool, _handle) = if is_sqlite {
            (sqlite_pool().await, None)
        } else {
            let (p, h) = rocksdb_pool();
            (p, Some(h))
        };

        let app = build_router(pool);

        // Register
        let mut t = Vec::with_capacity(n);
        for i in 0..n {
            let body = format!("name=U{}&email=u{}@m.co&password=pass123456&password_confirmation=pass123456", i, i);
            let req = Request::post("/register")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(body)).unwrap();
            let s = Instant::now();
            let _ = app.clone().oneshot(req).await.unwrap();
            t.push(s.elapsed());
        }
        print_row("Register (POST)", n, &t);

        // Login + collect sessions
        let mut lt = Vec::with_capacity(n);
        let mut sessions = Vec::with_capacity(n);
        for i in 0..n {
            let body = format!("email=u{}@m.co&password=pass123456", i);
            let req = Request::post("/login")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(body)).unwrap();
            let s = Instant::now();
            let resp = app.clone().oneshot(req).await.unwrap();
            lt.push(s.elapsed());
            let c = resp.headers()
                .get("set-cookie")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .split(';').next().unwrap_or("").to_string();
            sessions.push(c);
        }
        print_row("Login (POST)", n, &lt);

        // Dashboard
        let mut dt = Vec::with_capacity(n);
        for c in &sessions {
            let req = Request::get("/dashboard")
                .header("cookie", c)
                .body(Body::empty()).unwrap();
            let s = Instant::now();
            let _ = app.clone().oneshot(req).await.unwrap();
            dt.push(s.elapsed());
        }
        print_row("Dashboard (GET)", n, &dt);

        // Check if auth actually works
        let check = app.clone().oneshot(
            Request::get("/dashboard").header("cookie", &sessions[0]).body(Body::empty()).unwrap()
        ).await.unwrap();
        if check.status() != 200 {
            println!("    ⚠ Dashboard returned {} (auth failed)", check.status());
        }

        // Profile GET
        let mut pt = Vec::with_capacity(n);
        for c in &sessions {
            let req = Request::get("/profile")
                .header("cookie", c)
                .body(Body::empty()).unwrap();
            let s = Instant::now();
            let resp = app.clone().oneshot(req).await.unwrap();
            pt.push(s.elapsed());
            // Verify each request is 200 (not redirect from failed auth)
            assert_eq!(resp.status(), 200,
                "Profile GET returned {} — auth middleware not working?", resp.status());
        }
        print_row("Profile (GET)", n, &pt);

        // Verify Dashboard also 200
        let req = Request::get("/dashboard")
            .header("cookie", &sessions[0])
            .body(Body::empty()).unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), 200,
            "Dashboard returned {} — auth middleware not working?", resp.status());

        println!();
    }
}
