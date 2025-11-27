//! # avila-organism
//!
//! **Organismo de Email - Plataforma Completa**
//!
//! Um organismo é um sistema vivo completo, composto por múltiplos órgãos
//! trabalhando em harmonia. A plataforma de email Ávila é um "organismo digital"
//! totalmente integrado:
//!
//! - **Webmail** - Interface web moderna
//! - **API REST** - Integração com outros sistemas
//! - **CLI** - Ferramentas de linha de comando
//! - **Admin Panel** - Gerenciamento de usuários
//! - **Monitoring** - Observabilidade completa
//!
//! ## Filosofia
//!
//! Construído do zero, das partículas subatômicas (avila-nucleus) até um
//! organismo completo (avila-organism), sem dependências externas no core.
//!
//! **Do nêutron ao sistema de email completo - 100% Ávila Platform.**

pub mod webmail;
pub mod api;
pub mod admin;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Configuração da plataforma
#[derive(Debug, Clone)]
pub struct PlatformConfig {
    pub smtp_port: u16,
    pub imap_port: u16,
    pub http_port: u16,
    pub domain: String,
    pub data_dir: String,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            smtp_port: 2525,
            imap_port: 1143,
            http_port: 8080,
            domain: "localhost".to_string(),
            data_dir: "./data".to_string(),
        }
    }
}
