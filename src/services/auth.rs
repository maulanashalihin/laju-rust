use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand_core::OsRng;
use crate::models::User;
use crate::app::DbPool;
use crate::repositories::{UserRepository, SessionRepository, RocksDbUserRepository, RocksDbSessionRepository, SqliteUserRepository, SqliteSessionRepository};

pub struct AuthService {
    user_repo: Box<dyn UserRepository>,
    session_repo: Box<dyn SessionRepository>,
}

impl AuthService {
    pub fn new(pool: &DbPool) -> Self {
        match pool {
            DbPool::RocksDb(db) => Self {
                user_repo: Box::new(RocksDbUserRepository::new(db.clone())),
                session_repo: Box::new(RocksDbSessionRepository::new(db.clone())),
            },
            DbPool::Sqlite(pool) => Self {
                user_repo: Box::new(SqliteUserRepository::new(pool.clone())),
                session_repo: Box::new(SqliteSessionRepository::new(pool.clone())),
            },
        }
    }

    pub async fn register(&self, name: &str, email: &str, password: &str) -> Result<User, String> {
        if self.user_repo.find_id_by_email(email).await?.is_some() {
            return Err("Email already registered".into());
        }
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Hash error: {}", e))?
            .to_string();
        let user_id = uuid::Uuid::new_v4().to_string();
        let user = User::new(user_id, name.into(), email.into(), password_hash);
        self.user_repo.save(&user).await?;
        Ok(user)
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(String, User), String> {
        let user_id = self.user_repo.find_id_by_email(email).await?
            .ok_or("Invalid email or password".to_string())?;
        let user = self.user_repo.find_by_id(&user_id).await?
            .ok_or("Invalid email or password".to_string())?;
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| format!("Hash parse: {}", e))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| "Invalid email or password".to_string())?;
        let session = self.session_repo.create(&user.id).await?;
        Ok((session.id, user))
    }

    pub async fn logout(&self, session_id: &str) -> Result<(), String> {
        self.session_repo.delete(session_id).await
    }

    pub async fn get_user_by_session(&self, session_id: &str) -> Result<Option<User>, String> {
        let session = match self.session_repo.find_by_id(session_id).await? {
            Some(s) => s,
            None => return Ok(None),
        };
        self.user_repo.find_by_id(&session.user_id).await
    }

    pub async fn update_profile(&self, user_id: &str, name: &str, email: &str) -> Result<User, String> {
        let mut user = self.user_repo.find_by_id(user_id).await?
            .ok_or("User not found".to_string())?;
        if email != user.email {
            if let Some(existing_id) = self.user_repo.find_id_by_email(email).await? {
                if existing_id != user_id {
                    return Err("Email sudah terdaftar".into());
                }
            }
        }
        let old_email = user.email.clone();
        user.name = name.into();
        user.email = email.into();
        self.user_repo.update(&user, &old_email).await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::DbPool;
    use std::sync::Arc;

    fn test_pool() -> DbPool {
        let dir = tempfile::tempdir().unwrap();
        let db = Arc::new(rocksdb::DB::open_default(dir.path()).unwrap());
        DbPool::RocksDb(db)
    }

    #[test]
    fn test_register_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let svc = AuthService::new(&test_pool());
        let user = rt.block_on(svc.register("Alice", "alice@mail.com", "pass123")).unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@mail.com");
    }

    #[test]
    fn test_register_duplicate_email() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let svc = AuthService::new(&test_pool());
        rt.block_on(svc.register("Alice", "same@mail.com", "pass123")).unwrap();
        let result = rt.block_on(svc.register("Bob", "same@mail.com", "pass456"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Email already registered");
    }

    #[test]
    fn test_login_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let svc = AuthService::new(&test_pool());
        rt.block_on(svc.register("Alice", "alice@mail.com", "pass123")).unwrap();
        let (session_id, user) = rt.block_on(svc.login("alice@mail.com", "pass123")).unwrap();
        assert_eq!(user.email, "alice@mail.com");
        assert!(!session_id.is_empty());
    }

    #[test]
    fn test_update_profile_change_name() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let svc = AuthService::new(&test_pool());
        rt.block_on(svc.register("Alice", "alice@mail.com", "pass123")).unwrap();
        let id = rt.block_on(svc.user_repo.find_id_by_email("alice@mail.com")).unwrap().unwrap();
        let updated = rt.block_on(svc.update_profile(&id, "Alice Updated", "alice@mail.com")).unwrap();
        assert_eq!(updated.name, "Alice Updated");
    }

    #[test]
    fn test_update_profile_change_email() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let svc = AuthService::new(&test_pool());
        let user = rt.block_on(svc.register("Alice", "alice@mail.com", "pass123")).unwrap();
        let updated = rt.block_on(svc.update_profile(&user.id, "Alice", "baru@mail.com")).unwrap();
        assert_eq!(updated.email, "baru@mail.com");
        let (_, u) = rt.block_on(svc.login("baru@mail.com", "pass123")).unwrap();
        assert_eq!(u.id, user.id);
        let result = rt.block_on(svc.login("alice@mail.com", "pass123"));
        assert!(result.is_err());
    }

    #[test]
    fn test_update_profile_duplicate_email() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let svc = AuthService::new(&test_pool());
        let alice = rt.block_on(svc.register("Alice", "alice@mail.com", "pass123")).unwrap();
        rt.block_on(svc.register("Bob", "bob@mail.com", "pass456")).unwrap();
        let result = rt.block_on(svc.update_profile(&alice.id, "Alice", "bob@mail.com"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Email sudah terdaftar");
    }
}
