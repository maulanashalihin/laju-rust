use crate::models::User;
use std::sync::Arc;
use rocksdb::DB;

/// Example service — business logic layer.
/// Services memisahkan logic dari handlers supaya bisa di-test dan di-reuse.
pub struct GreetService {
    pub db: Arc<DB>,
}

impl GreetService {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    /// Example: generate greeting message.
    pub fn greet(&self, name: &str) -> String {
        format!("Hello, {}! Welcome to Laju Rust.", name)
    }

        pub fn save_user(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("user:{}", user.id);
        let value = serde_json::to_vec(user)?;
        self.db.put(key.as_bytes(), &value)?;
        Ok(())
    }

    pub fn get_user(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let key = format!("user:{}", id);
        match self.db.get(key.as_bytes())? {
            Some(data) => {
                let user: User = serde_json::from_slice(&data)?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
}
