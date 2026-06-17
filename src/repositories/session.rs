use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use rocksdb::DB;
use uuid::Uuid;
use crate::models::Session;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, user_id: &str) -> Result<Session, String>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Session>, String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
}

// ── RocksDB implementation ──────────────────────────────────────────

pub struct RocksDbSessionRepository {
    pub db: Arc<DB>,
}

impl RocksDbSessionRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SessionRepository for RocksDbSessionRepository {
    async fn create(&self, user_id: &str) -> Result<Session, String> {
        let db = self.db.clone();
        let user_id = user_id.to_string();
        tokio::task::spawn_blocking(move || {
            let id = Uuid::new_v4().to_string();
            let session = Session { id: id.clone(), user_id, created_at: Utc::now().timestamp() };
            let key = format!("session:{}", id);
            let value = serde_json::to_vec(&session).map_err(|e| format!("Ser: {}", e))?;
            db.put(key.as_bytes(), &value).map_err(|e| format!("DB: {}", e))?;
            Ok(session)
        }).await.map_err(|e| format!("Join: {}", e))?
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Session>, String> {
        let db = self.db.clone();
        let id = id.to_string();
        tokio::task::spawn_blocking(move || {
            let key = format!("session:{}", id);
            match db.get(key.as_bytes()) {
                Ok(Some(data)) => Ok(Some(serde_json::from_slice(&data).map_err(|e| format!("Deser: {}", e))?)),
                Ok(None) => Ok(None),
                Err(e) => Err(format!("DB: {}", e)),
            }
        }).await.map_err(|e| format!("Join: {}", e))?
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let db = self.db.clone();
        let id = id.to_string();
        tokio::task::spawn_blocking(move || {
            db.delete(format!("session:{}", id).as_bytes()).map_err(|e| format!("DB: {}", e))
        }).await.map_err(|e| format!("Join: {}", e))?
    }
}

// ── SQLite implementation ──────────────────────────────────────────

pub struct SqliteSessionRepository {
    pub pool: sqlx::SqlitePool,
}

impl SqliteSessionRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionRepository for SqliteSessionRepository {
    async fn create(&self, user_id: &str) -> Result<Session, String> {
        let id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO sessions (id, user_id) VALUES (?, ?)")
            .bind(&id).bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("DB: {}", e))?;
        Ok(Session { id, user_id: user_id.into(), created_at: Utc::now().timestamp() })
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Session>, String> {
        let row: Option<(String, String, i64)> = sqlx::query_as(
            "SELECT id, user_id, strftime('%s', created_at) FROM sessions WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB: {}", e))?;
        Ok(row.map(|r| Session { id: r.0, user_id: r.1, created_at: r.2 }))
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("DB: {}", e))?;
        Ok(())
    }
}
