/// App configuration — dibaca dari environment variable dengan default.
#[derive(Clone, Debug)]
pub struct AppConfig {
    /// Development mode (default: true)
    pub dev_mode: bool,
    /// Database backend: "rocksdb" atau "sqlite" (default: "rocksdb")
    pub db_backend: String,
    /// Path ke RocksDB storage (default: "data/rocksdb")
    pub db_path: String,
    /// Server listen address (default: "0.0.0.0:3000")
    pub server_addr: String,
    /// Vite dev server URL (default: "http://localhost:5173")
    pub vite_dev_url: String,
}

impl AppConfig {
    /// Load config dari environment variable + fallback default.
    pub fn from_env() -> Self {
        Self {
            dev_mode: std::env::var("DEV_MODE")
                .ok()
                .and_then(|v| v.parse::<bool>().ok())
                .unwrap_or(true),
            db_backend: std::env::var("DB_BACKEND")
                .unwrap_or_else(|_| "rocksdb".into()),
            db_path: std::env::var("DB_PATH")
                .unwrap_or_else(|_| "data/rocksdb".into()),
            server_addr: std::env::var("SERVER_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:3000".into()),
            vite_dev_url: std::env::var("VITE_DEV_URL")
                .unwrap_or_else(|_| "http://localhost:5173".into()),
        }
    }
}
