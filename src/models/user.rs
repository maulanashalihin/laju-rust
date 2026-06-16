use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

impl User {
    pub fn new(id: String, name: String, email: String, password_hash: String) -> Self {
        Self { id, name, email, password_hash, role: "admin".into() }
    }
}
