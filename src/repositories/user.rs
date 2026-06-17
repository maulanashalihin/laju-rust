use std::sync::Arc;
use async_trait::async_trait;
use rocksdb::DB;
use crate::models::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_id_by_email(&self, email: &str) -> Result<Option<String>, String>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, String>;
    async fn save(&self, user: &User) -> Result<(), String>;
    async fn update(&self, user: &User, old_email: &str) -> Result<(), String>;
}

// ── RocksDB implementation ──────────────────────────────────────────

pub struct RocksDbUserRepository {
    pub db: Arc<DB>,
}

impl RocksDbUserRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for RocksDbUserRepository {
    async fn find_id_by_email(&self, email: &str) -> Result<Option<String>, String> {
        let db = self.db.clone();
        let email = email.to_string();
        tokio::task::spawn_blocking(move || {
            let key = format!("idx:email:{}", email);
            match db.get(key.as_bytes()) {
                Ok(Some(id)) => Ok(Some(String::from_utf8(id).map_err(|_| "Invalid id".to_string())?)),
                Ok(None) => Ok(None),
                Err(e) => Err(format!("DB: {}", e)),
            }
        }).await.map_err(|e| format!("Join: {}", e))?
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, String> {
        let db = self.db.clone();
        let id = id.to_string();
        tokio::task::spawn_blocking(move || {
            let key = format!("user:{}", id);
            match db.get(key.as_bytes()) {
                Ok(Some(data)) => Ok(Some(serde_json::from_slice(&data).map_err(|e| format!("Deser: {}", e))?)),
                Ok(None) => Ok(None),
                Err(e) => Err(format!("DB: {}", e)),
            }
        }).await.map_err(|e| format!("Join: {}", e))?
    }

    async fn save(&self, user: &User) -> Result<(), String> {
        let db = self.db.clone();
        let user = user.clone();
        tokio::task::spawn_blocking(move || {
            let key = format!("user:{}", user.id);
            let value = serde_json::to_vec(&user).map_err(|e| format!("Ser: {}", e))?;
            db.put(key.as_bytes(), &value).map_err(|e| format!("DB: {}", e))?;
            let idx = format!("idx:email:{}", user.email);
            db.put(idx.as_bytes(), user.id.as_bytes()).map_err(|e| format!("DB idx: {}", e))?;
            Ok(())
        }).await.map_err(|e| format!("Join: {}", e))?
    }

    async fn update(&self, user: &User, old_email: &str) -> Result<(), String> {
        let db = self.db.clone();
        let user = user.clone();
        let old_email = old_email.to_string();
        tokio::task::spawn_blocking(move || {
            let key = format!("user:{}", user.id);
            let value = serde_json::to_vec(&user).map_err(|e| format!("Ser: {}", e))?;
            db.put(key.as_bytes(), &value).map_err(|e| format!("DB: {}", e))?;
            if old_email != user.email {
                db.delete(format!("idx:email:{}", old_email).as_bytes())
                    .map_err(|e| format!("DB: {}", e))?;
                db.put(format!("idx:email:{}", user.email).as_bytes(), user.id.as_bytes())
                    .map_err(|e| format!("DB: {}", e))?;
            }
            Ok(())
        }).await.map_err(|e| format!("Join: {}", e))?
    }
}

// ── SQLite implementation ──────────────────────────────────────────

pub struct SqliteUserRepository {
    pub pool: sqlx::SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn find_id_by_email(&self, email: &str) -> Result<Option<String>, String> {
        let row: Option<(String,)> = sqlx::query_as("SELECT id FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("DB: {}", e))?;
        Ok(row.map(|r| r.0))
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, String> {
        let row: Option<(String, String, String, String, String)> = sqlx::query_as(
            "SELECT id, name, email, password_hash, role FROM users WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB: {}", e))?;
        Ok(row.map(|r| User {
            id: r.0, name: r.1, email: r.2, password_hash: r.3, role: r.4,
        }))
    }

    async fn save(&self, user: &User) -> Result<(), String> {
        sqlx::query("INSERT INTO users (id, name, email, password_hash, role) VALUES (?, ?, ?, ?, ?)")
            .bind(&user.id).bind(&user.name).bind(&user.email)
            .bind(&user.password_hash).bind(&user.role)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("DB: {}", e))?;
        Ok(())
    }

    async fn update(&self, user: &User, _old_email: &str) -> Result<(), String> {
        sqlx::query("UPDATE users SET name = ?, email = ? WHERE id = ?")
            .bind(&user.name).bind(&user.email).bind(&user.id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("DB: {}", e))?;
        Ok(())
    }
}
