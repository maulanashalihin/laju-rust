use std::sync::Arc;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand_core::OsRng;
use rocksdb::DB;
use crate::models::User;
use crate::repositories::{UserRepository, SessionRepository};

pub struct AuthService {
    user_repo: UserRepository,
    session_repo: SessionRepository,
}

impl AuthService {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            user_repo: UserRepository::new(db.clone()),
            session_repo: SessionRepository::new(db),
        }
    }

    pub fn register(&self, name: &str, email: &str, password: &str) -> Result<User, String> {
        if self.user_repo.find_id_by_email(email)?.is_some() {
            return Err("Email already registered".into());
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Hash error: {}", e))?
            .to_string();

        let user_id = uuid::Uuid::new_v4().to_string();
        let user = User::new(user_id, name.into(), email.into(), password_hash);
        self.user_repo.save(&user)?;
        Ok(user)
    }

    pub fn login(&self, email: &str, password: &str) -> Result<(String, User), String> {
        let user_id = self.user_repo.find_id_by_email(email)?
            .ok_or("Invalid email or password".to_string())?;
        let user = self.user_repo.find_by_id(&user_id)?
            .ok_or("Invalid email or password".to_string())?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| format!("Hash parse: {}", e))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| "Invalid email or password".to_string())?;

        let session = self.session_repo.create(&user.id)?;
        Ok((session.id, user))
    }

    pub fn logout(&self, session_id: &str) -> Result<(), String> {
        self.session_repo.delete(session_id)
    }

    pub fn update_profile(&self, user_id: &str, name: &str, email: &str) -> Result<User, String> {
        let mut user = self.user_repo.find_by_id(user_id)?
            .ok_or("User not found".to_string())?;

        if email != user.email {
            if let Some(existing_id) = self.user_repo.find_id_by_email(email)? {
                if existing_id != user_id {
                    return Err("Email sudah terdaftar".into());
                }
            }
        }

        let old_email = user.email.clone();
        user.name = name.into();
        user.email = email.into();
        self.user_repo.update(&user, &old_email)?;
        Ok(user)
    }

    pub fn get_user_by_session(&self, session_id: &str) -> Result<Option<User>, String> {
        let session = match self.session_repo.find_by_id(session_id)? {
            Some(s) => s,
            None => return Ok(None),
        };
        self.user_repo.find_by_id(&session.user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_svc() -> AuthService {
        let dir = tempfile::tempdir().unwrap();
        let db = Arc::new(rocksdb::DB::open_default(dir.path()).unwrap());
        AuthService::new(db)
    }

    #[test]
    fn test_register_success() {
        let svc = test_svc();
        let user = svc.register("Alice", "alice@mail.com", "pass123").unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@mail.com");
    }

    #[test]
    fn test_register_duplicate_email() {
        let svc = test_svc();
        svc.register("Alice", "same@mail.com", "pass123").unwrap();
        let result = svc.register("Bob", "same@mail.com", "pass456");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Email already registered");
    }

    #[test]
    fn test_login_success() {
        let svc = test_svc();
        svc.register("Alice", "alice@mail.com", "pass123").unwrap();
        let (session_id, user) = svc.login("alice@mail.com", "pass123").unwrap();
        assert_eq!(user.email, "alice@mail.com");
        assert!(!session_id.is_empty());
    }

    #[test]
    fn test_login_wrong_password() {
        let svc = test_svc();
        svc.register("Alice", "alice@mail.com", "pass123").unwrap();
        let result = svc.login("alice@mail.com", "wrong");
        assert!(result.is_err());
    }

    #[test]
    fn test_login_nonexistent_email() {
        let svc = test_svc();
        let result = svc.login("unknown@mail.com", "pass123");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_profile_change_name() {
        let svc = test_svc();
        svc.register("Alice", "alice@mail.com", "pass123").unwrap();

        let updated = svc.update_profile(
            svc.user_repo.find_id_by_email("alice@mail.com").unwrap().unwrap().as_str(),
            "Alice Updated",
            "alice@mail.com",
        ).unwrap();
        assert_eq!(updated.name, "Alice Updated");
    }

    #[test]
    fn test_update_profile_change_email() {
        let svc = test_svc();
        let user = svc.register("Alice", "alice@mail.com", "pass123").unwrap();

        let updated = svc.update_profile(&user.id, "Alice", "baru@mail.com").unwrap();
        assert_eq!(updated.email, "baru@mail.com");

        let (_, u) = svc.login("baru@mail.com", "pass123").unwrap();
        assert_eq!(u.id, user.id);

        let result = svc.login("alice@mail.com", "pass123");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_profile_duplicate_email() {
        let svc = test_svc();
        let alice = svc.register("Alice", "alice@mail.com", "pass123").unwrap();
        svc.register("Bob", "bob@mail.com", "pass456").unwrap();

        let result = svc.update_profile(&alice.id, "Alice", "bob@mail.com");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Email sudah terdaftar");
    }
}
