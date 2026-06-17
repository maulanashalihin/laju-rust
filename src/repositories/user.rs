use std::sync::Arc;
use rocksdb::DB;
use crate::models::User;

pub struct UserRepository {
    pub db: Arc<DB>,
}

impl UserRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    /// Cari user berdasarkan email via index key `idx:email:<email>`.
    pub fn find_id_by_email(&self, email: &str) -> Result<Option<String>, String> {
        let key = format!("idx:email:{}", email);
        match self.db.get(key.as_bytes()) {
            Ok(Some(id)) => Ok(Some(String::from_utf8(id).map_err(|_| "Invalid id".to_string())?)),
            Ok(None) => Ok(None),
            Err(e) => Err(format!("DB: {}", e)),
        }
    }

    /// Cari user berdasarkan primary key `user:<id>`.
    pub fn find_by_id(&self, id: &str) -> Result<Option<User>, String> {
        let key = format!("user:{}", id);
        match self.db.get(key.as_bytes()) {
            Ok(Some(data)) => {
                let user: User = serde_json::from_slice(&data)
                    .map_err(|e| format!("Deser user: {}", e))?;
                Ok(Some(user))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("DB: {}", e)),
        }
    }

    /// Simpan user + update email index. Atomic di level service (bukan transactional).
    pub fn save(&self, user: &User) -> Result<(), String> {
        let key = format!("user:{}", user.id);
        let value = serde_json::to_vec(user).map_err(|e| format!("Ser: {}", e))?;
        self.db.put(key.as_bytes(), &value).map_err(|e| format!("DB: {}", e))?;
        let idx_key = format!("idx:email:{}", user.email);
        self.db.put(idx_key.as_bytes(), user.id.as_bytes())
            .map_err(|e| format!("DB idx: {}", e))?;
        Ok(())
    }

    pub fn update(&self, user: &User, old_email: &str) -> Result<(), String> {
        let key = format!("user:{}", user.id);
        let value = serde_json::to_vec(user).map_err(|e| format!("Ser: {}", e))?;
        self.db.put(key.as_bytes(), &value).map_err(|e| format!("DB: {}", e))?;
        if old_email != user.email {
            self.db.delete(format!("idx:email:{}", old_email).as_bytes())
                .map_err(|e| format!("DB: {}", e))?;
            self.db.put(format!("idx:email:{}", user.email).as_bytes(), user.id.as_bytes())
                .map_err(|e| format!("DB: {}", e))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::User;

    fn test_db() -> Arc<DB> {
        let dir = tempfile::tempdir().unwrap();
        Arc::new(DB::open_default(dir.path()).unwrap())
    }

    fn make_user(id: &str, email: &str) -> User {
        User::new(id.into(), format!("User {}", id), email.into(), "hash".into())
    }

    #[test]
    fn test_save_and_find_by_id() {
        let repo = UserRepository::new(test_db());
        let user = make_user("1", "a@mail.com");
        repo.save(&user).unwrap();

        let found = repo.find_by_id("1").unwrap().unwrap();
        assert_eq!(found.name, "User 1");
        assert_eq!(found.email, "a@mail.com");
    }

    #[test]
    fn test_find_not_found() {
        let repo = UserRepository::new(test_db());
        let result = repo.find_by_id("nonexistent").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_find_id_by_email() {
        let repo = UserRepository::new(test_db());
        repo.save(&make_user("1", "a@mail.com")).unwrap();
        repo.save(&make_user("2", "b@mail.com")).unwrap();

        let id = repo.find_id_by_email("a@mail.com").unwrap().unwrap();
        assert_eq!(id, "1");

        let none = repo.find_id_by_email("x@mail.com").unwrap();
        assert!(none.is_none());
    }

    #[test]
    fn test_update_email_hapus_index_lama() {
        let repo = UserRepository::new(test_db());
        let mut user = make_user("1", "lama@mail.com");
        repo.save(&user).unwrap();

        let old_email = user.email.clone();
        user.email = "baru@mail.com".into();
        repo.update(&user, &old_email).unwrap();

        // Index lama harus ilang
        let lama = repo.find_id_by_email("lama@mail.com").unwrap();
        assert!(lama.is_none());

        let baru = repo.find_id_by_email("baru@mail.com").unwrap();
        assert_eq!(baru, Some("1".into()));
    }
}
