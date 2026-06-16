use std::sync::Arc;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand_core::OsRng;
use rocksdb::DB;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{User, Session};

pub struct AuthService {
    pub db: Arc<DB>,
}

impl AuthService {
    pub fn new(db: Arc<DB>) -> Self { Self { db } }

    pub fn register(&self, name: &str, email: &str, password: &str) -> Result<User, String> {
        let email_key = format!("idx:email:{}", email);
        if let Ok(Some(_)) = self.db.get(email_key.as_bytes()) {
            return Err("Email already registered".into());
        }
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Hash error: {}", e))?
            .to_string();
        let user_id = Uuid::new_v4().to_string();
        let user = User::new(user_id.clone(), name.into(), email.into(), password_hash);
        let user_key = format!("user:{}", user.id);
        let user_value = serde_json::to_vec(&user).map_err(|e| format!("Ser: {}", e))?;
        self.db.put(user_key.as_bytes(), &user_value).map_err(|e| format!("DB: {}", e))?;
        self.db.put(email_key.as_bytes(), user_id.as_bytes()).map_err(|e| format!("DB: {}", e))?;
        Ok(user)
    }

    pub fn login(&self, email: &str, password: &str) -> Result<(String, User), String> {
        let email_key = format!("idx:email:{}", email);
        let user_id = match self.db.get(email_key.as_bytes()) {
            Ok(Some(id)) => String::from_utf8(id).map_err(|_| "Invalid id".to_string())?,
            _ => return Err("Invalid email or password".into()),
        };
        let user_key = format!("user:{}", user_id);
        let user_data = self.db.get(user_key.as_bytes())
            .map_err(|e| format!("DB: {}", e))?
            .ok_or("User not found".to_string())?;
        let user: User = serde_json::from_slice(&user_data).map_err(|e| format!("Deser: {}", e))?;
        let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| format!("Hash: {}", e))?;
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| "Invalid email or password".to_string())?;
        let session_id = Uuid::new_v4().to_string();
        let session = Session { id: session_id.clone(), user_id: user.id.clone(), created_at: Utc::now().timestamp() };
        let session_key = format!("session:{}", session_id);
        let sv = serde_json::to_vec(&session).map_err(|e| format!("Ser: {}", e))?;
        self.db.put(session_key.as_bytes(), &sv).map_err(|e| format!("DB: {}", e))?;
        Ok((session_id, user))
    }

    pub fn logout(&self, session_id: &str) -> Result<(), String> {
        self.db.delete(format!("session:{}", session_id).as_bytes())
            .map_err(|e| format!("DB: {}", e))
    }

    pub fn get_user_by_session(&self, session_id: &str) -> Result<Option<User>, String> {
        let session_key = format!("session:{}", session_id);
        let session_data = match self.db.get(session_key.as_bytes()) {
            Ok(Some(d)) => d, Ok(None) => return Ok(None),
            Err(e) => return Err(format!("DB: {}", e)),
        };
        let session: Session = serde_json::from_slice(&session_data).map_err(|e| format!("Deser: {}", e))?;
        let user_key = format!("user:{}", session.user_id);
        let user_data = match self.db.get(user_key.as_bytes()) {
            Ok(Some(d)) => d, Ok(None) => return Ok(None),
            Err(e) => return Err(format!("DB: {}", e)),
        };
        let user: User = serde_json::from_slice(&user_data).map_err(|e| format!("Deser: {}", e))?;
        Ok(Some(user))
    }
}
