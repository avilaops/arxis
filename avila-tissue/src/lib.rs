//! # avila-tissue
//!
//! **Tecido Digital - Organização de Emails**
//!
//! Assim como tecidos biológicos organizam células em estruturas funcionais,
//! esta biblioteca organiza emails (células digitais) em um sistema coerente:
//!
//! - **Storage** - Armazenamento persistente de emails
//! - **Indexing** - Índices de busca full-text
//! - **Searching** - Busca rápida e relevante
//! - **Organization** - Pastas, tags, threads
//!
//! ## Filosofia
//!
//! Tecidos são conjuntos organizados de células que trabalham juntas.
//! O sistema de email precisa organizar milhões de mensagens de forma
//! eficiente, pesquisável e confiável.

#![warn(missing_docs)]

use avila_error::{Error, ErrorKind, Result};
use avila_cell::message::Email;
use avila_id::Id;
use avila_time::DateTime;

pub mod storage;
pub mod index;
pub mod search;
pub mod mailbox;

/// Versão da biblioteca
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Metadados de email armazenado
#[derive(Debug, Clone)]
pub struct EmailMetadata {
    /// ID único
    pub id: Id,
    /// Data de recebimento
    pub received_at: DateTime,
    /// Tamanho em bytes
    pub size: usize,
    /// Flags (read, starred, etc)
    pub flags: Vec<String>,
    /// Mailbox onde está armazenado
    pub mailbox: String,
    /// Thread ID (para conversas)
    pub thread_id: Option<Id>,
}

/// Flags de email
pub mod flags {
    /// Email foi lido
    pub const SEEN: &str = "\\Seen";
    /// Email tem resposta
    pub const ANSWERED: &str = "\\Answered";
    /// Email está marcado
    pub const FLAGGED: &str = "\\Flagged";
    /// Email será deletado
    pub const DELETED: &str = "\\Deleted";
    /// Email é rascunho
    pub const DRAFT: &str = "\\Draft";
    /// Email é recente
    pub const RECENT: &str = "\\Recent";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_creation() {
        let meta = EmailMetadata {
            id: Id::new(),
            received_at: DateTime::now(),
            size: 1024,
            flags: vec![flags::SEEN.to_string()],
            mailbox: "INBOX".to_string(),
            thread_id: None,
        };

        assert!(meta.flags.contains(&flags::SEEN.to_string()));
    }
}
