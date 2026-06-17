use std::sync::Arc;
use chrono::Utc;
use rocksdb::DB;
use uuid::Uuid;
use crate::models::Session;

pub struct SessionRepository {
    pub db: Arc<DB>,
}

impl SessionRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    /// Buat session baru, simpan dengan key `session:<id>`.
    pub fn create(&self, user_id: &str) -> Result<Session, String> {
        let id = Uuid::new_v4().to_string();
        let session = Session {
            id: id.clone(),
            user_id: user_id.into(),
            created_at: Utc::now().timestamp(),
        };
        let key = format!("session:{}", id);
        let value = serde_json::to_vec(&session).map_err(|e| format!("Ser: {}", e))?;
        self.db.put(key.as_bytes(), &value).map_err(|e| format!("DB: {}", e))?;
        Ok(session)
    }

    /// Cari session berdasarkan primary key `session:<id>`.
    pub fn find_by_id(&self, id: &str) -> Result<Option<Session>, String> {
        let key = format!("session:{}", id);
        match self.db.get(key.as_bytes()) {
            Ok(Some(data)) => {
                let session: Session = serde_json::from_slice(&data)
                    .map_err(|e| format!("Deser session: {}", e))?;
                Ok(Some(session))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("DB: {}", e)),
        }
    }

    /// Hapus session berdasarkan id.
    pub fn delete(&self, id: &str) -> Result<(), String> {
        self.db.delete(format!("session:{}", id).as_bytes())
            .map_err(|e| format!("DB: {}", e))
    }
}
