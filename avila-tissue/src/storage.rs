//! Armazenamento persistente de emails

use crate::{EmailMetadata, flags};
use avila_error::{Error, ErrorKind, Result};
use avila_cell::message::Email;
use avila_id::Id;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Storage de emails em memória (HashMap)
pub struct EmailStorage {
    emails: Arc<RwLock<HashMap<String, Email>>>,
}

impl EmailStorage {
    /// Cria novo storage em memória
    pub fn new() -> Self {
        Self {
            emails: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Armazena um email
    pub fn store(&self, email: &Email, metadata: &EmailMetadata) -> Result<()> {
        let mut emails = self.emails.write()
            .map_err(|_| Error::new(ErrorKind::Internal, "Failed to lock"))?;

        emails.insert(metadata.id.to_string(), email.clone());
        Ok(())
    }

    /// Recupera um email por ID
    pub fn get(&self, id: &Id) -> Result<Option<Email>> {
        let emails = self.emails.read()
            .map_err(|_| Error::new(ErrorKind::Internal, "Failed to lock"))?;

        Ok(emails.get(&id.to_string()).cloned())
    }

    /// Deleta um email
    pub fn delete(&self, id: &Id) -> Result<()> {
        let mut emails = self.emails.write()
            .map_err(|_| Error::new(ErrorKind::Internal, "Failed to lock"))?;

        emails.remove(&id.to_string());
        Ok(())
    }

    /// Lista todos os IDs
    pub fn list_ids(&self) -> Result<Vec<Id>> {
        let emails = self.emails.read()
            .map_err(|_| Error::new(ErrorKind::Internal, "Failed to lock"))?;

        let mut ids = Vec::new();
        for key in emails.keys() {
            if let Ok(id) = Id::parse(key) {
                ids.push(id);
            }
        }
        Ok(ids)
    }
}

impl Default for EmailStorage {
    fn default() -> Self {
        Self::new()
    }
}
