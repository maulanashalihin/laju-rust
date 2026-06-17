use std::sync::Arc;
use std::time::Instant;
use axum::body::Body;
use axum::http::Request;
use tower::ServiceExt;
use laju_rust::app::{AppState, DbPool};
use laju_rust::config::AppConfig;
use laju_rust::routes;

fn build_router(db: DbPool) -> axum::Router {
    let mut config = AppConfig::from_env();
    config.dev_mode = true;

    let state = Arc::new(AppState { db, config: config.clone() });

    // Minimal InertiaConfig — tidak perlu real template untuk benchmark
    let layout = move |_: String| -> String { String::new() };
    let inertia_cfg = axum_inertia::InertiaConfig::new(None::<String>, Box::new(layout));

    routes::register(axum::Router::new())
        .layer(axum::Extension(state))
        .with_state(inertia_cfg)
}

async fn rocksdb_pool() -> DbPool {
    let dir = tempfile::tempdir().unwrap();
    let mut opts = rocksdb::Options::default();
    opts.create_if_missing(true);
    let db = Arc::new(rocksdb::DB::open(&opts, dir.path()).unwrap());
    DbPool::RocksDb(db)
}

async fn sqlite_pool() -> DbPool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
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
            id TEXT PRIMARY KEY, user_id TEXT NOT NULL REFERENCES users(id)
        )"
    ).execute(&pool).await.unwrap();
    DbPool::Sqlite(pool)
}

fn op(label: &str, count: usize, trips: &[std::time::Duration]) {
    let total: std::time::Duration = trips.iter().sum();
    let ops_s = if total.as_secs_f64() > 0.0 {
        (count as f64 / total.as_secs_f64()) as u64
    } else { 0 };

    let mut sorted = trips.to_vec();
    sorted.sort();
    let p50 = sorted[sorted.len() / 2];
    let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
    let p99 = sorted[(sorted.len() as f64 * 0.99) as usize];

    let fmt = |n: u64| {
        if n >= 1_000_000 { format!("{:.1}M", n as f64 / 1_000_000.0) }
        else if n >= 1_000 { format!("{:.0}K", n as f64 / 1_000.0) }
        else { n.to_string() }
    };

    println!("  {:<25} {:>8} ops/s   P50:{:>6}ms P95:{:>6}ms P99:{:>6}ms",
        label, fmt(ops_s),
        p50.as_millis(), p95.as_millis(), p99.as_millis());
}

struct Session {
    pub email: String,
    pub cookie: String,
}

async fn bench_backend(_label: &str, pool: DbPool, iterations: usize) {
    let app = build_router(pool);

    // --- Register ---
    let mut reg_trips = Vec::with_capacity(iterations);
    for i in 0..iterations {
        let body = format!("name=User{}&email=user{}@mail.com&password=pass123456&password_confirmation=pass123456", i, i);
        let t = Instant::now();
        let req = Request::post("/register")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body)).unwrap();
        let _ = app.clone().oneshot(req).await.unwrap();
        reg_trips.push(t.elapsed());
    }
    op("Register (POST)", iterations, &reg_trips);

    // --- Login untuk session -- ambil satu session buat test authenticated routes
    let mut login_trips = Vec::with_capacity(iterations);
    let mut sessions = Vec::with_capacity(iterations);
    for i in 0..iterations {
        let body = format!("email=user{}@mail.com&password=pass123456", i);
        let t = Instant::now();
        let req = Request::post("/login")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body)).unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        let elapsed = t.elapsed();
        login_trips.push(elapsed);

        // Extract session cookie
        let cookie = resp.headers()
            .get("set-cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .split(';')
            .next()
            .unwrap_or("")
            .to_string();
        sessions.push(Session { email: format!("user{}@mail.com", i), cookie });
    }
    op("Login (POST)", iterations, &login_trips);

    // --- Dashboard (GET with session) ---
    let mut dash_trips = Vec::with_capacity(iterations);
    for sess in &sessions {
        let t = Instant::now();
        let req = Request::get("/dashboard")
            .header("cookie", &sess.cookie)
            .body(Body::empty()).unwrap();
        let _ = app.clone().oneshot(req).await.unwrap();
        dash_trips.push(t.elapsed());
    }
    op("Dashboard (GET)", iterations, &dash_trips);

    // --- Profile GET ---
    let mut prof_trips = Vec::with_capacity(iterations);
    for sess in &sessions {
        let t = Instant::now();
        let req = Request::get("/profile")
            .header("cookie", &sess.cookie)
            .body(Body::empty()).unwrap();
        let _ = app.clone().oneshot(req).await.unwrap();
        prof_trips.push(t.elapsed());
    }
    op("Profile (GET)", iterations, &prof_trips);

    // --- Profile Update (POST) ---
    let mut up_trips = Vec::with_capacity(iterations);
    for (i, sess) in sessions.iter().enumerate() {
        let body = format!("name=Updated{}&email={}", i, sess.email);
        let t = Instant::now();
        let req = Request::post("/profile")
            .header("content-type", "application/x-www-form-urlencoded")
            .header("cookie", &sess.cookie)
            .body(Body::from(body)).unwrap();
        let _ = app.clone().oneshot(req).await.unwrap();
        up_trips.push(t.elapsed());
    }
    op("Profile Update (POST)", iterations, &up_trips);

    println!();
}

#[tokio::main]
async fn main() {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║     Laju Rust — HTTP Endpoint Benchmark    ║");
    println!("╚══════════════════════════════════════════════╝\n");

    let iterations = 500;

    println!("── RocksDB ────────────────────────────────\n");
    bench_backend("RocksDB", rocksdb_pool().await, iterations).await;

    println!("── SQLite (via sqlx) ────────────────────────\n");
    bench_backend("SQLite", sqlite_pool().await, iterations).await;
}
