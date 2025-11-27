//! Sistema de autenticação

use avila_error::Result;
use std::collections::HashMap;

pub struct AuthSystem {
    users: HashMap<String, String>, // username -> password_hash
}

impl AuthSystem {
    pub fn new() -> Self {
        Self { users: HashMap::new() }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<bool> {
        // TODO: Implementar autenticação real com bcrypt
        Ok(self.users.get(username).map(|p| p == password).unwrap_or(false))
    }
}
