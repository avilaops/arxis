//! # Avila Error - Sistema de Erros Unificado (Layer 1)
//!
//! Sistema de erros hierárquico 100% manual, sem std::error::Error.
//!
//! ## Características
//! - `#![no_std]` por padrão (com feature `alloc`)
//! - Sistema de códigos de erro (u32)
//! - Contexto manual de erros
//! - Macros `bail!` e `ensure!`
//! - ZERO uso de `std::error::Error`
//! - ZERO uso de `core::fmt::Display`
//! - Formatação manual de mensagens

#![no_std]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Tipo Result customizado usando AvilaError
pub type Result<T> = core::result::Result<T, AvilaError>;

/// Sistema de erros hierárquico do Avila
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvilaError {
    /// Código numérico do erro (para interoperabilidade)
    code: u32,
    /// Tipo/categoria do erro
    kind: ErrorKind,
    /// Mensagem descritiva (opcional com feature alloc)
    #[cfg(feature = "alloc")]
    message: Option<String>,
    /// Contexto adicional (pilha de contextos)
    #[cfg(feature = "alloc")]
    context: Vec<String>,
}

/// Categorias de erro (10+ variantes conforme requisito)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrorKind {
    /// Erro de entrada inválida
    InvalidInput = 1,
    /// Recurso não encontrado
    NotFound = 2,
    /// Permissão negada
    PermissionDenied = 3,
    /// Falha de conexão
    ConnectionFailed = 4,
    /// Timeout de operação
    Timeout = 5,
    /// Dados corrompidos
    DataCorruption = 6,
    /// Erro de configuração
    ConfigError = 7,
    /// Falha de autenticação
    AuthenticationFailed = 8,
    /// Falha de autorização
    AuthorizationFailed = 9,
    /// Recurso já existe
    AlreadyExists = 10,
    /// Recurso esgotado
    ResourceExhausted = 11,
    /// Operação cancelada
    Cancelled = 12,
    /// Erro interno
    Internal = 13,
    /// Não implementado
    NotImplemented = 14,
    /// Serviço indisponível
    Unavailable = 15,
    /// Erro desconhecido
    Unknown = 255,
}

impl AvilaError {
    /// Cria um novo erro com código e tipo
    #[inline]
    pub const fn new(code: u32, kind: ErrorKind) -> Self {
        Self {
            code,
            kind,
            #[cfg(feature = "alloc")]
            message: None,
            #[cfg(feature = "alloc")]
            context: Vec::new(),
        }
    }

    /// Cria um erro com mensagem
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn with_message(code: u32, kind: ErrorKind, message: String) -> Self {
        Self {
            code,
            kind,
            message: Some(message),
            context: Vec::new(),
        }
    }

    /// Adiciona contexto ao erro
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn add_context(mut self, ctx: String) -> Self {
        self.context.push(ctx);
        self
    }

    /// Retorna o código do erro
    #[inline]
    pub const fn code(&self) -> u32 {
        self.code
    }

    /// Retorna o tipo do erro
    #[inline]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Retorna a mensagem do erro (se disponível)
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Retorna o contexto do erro
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn context(&self) -> &[String] {
        &self.context
    }

    /// Formata o erro manualmente para um buffer
    #[cfg(feature = "alloc")]
    pub fn format(&self) -> String {
        use alloc::format;
        
        let mut output = format!("[{}:{}] ", self.code, self.kind.as_str());
        
        if let Some(ref msg) = self.message {
            output.push_str(msg);
        } else {
            output.push_str(self.kind.default_message());
        }
        
        if !self.context.is_empty() {
            output.push_str("\nContext:");
            for (i, ctx) in self.context.iter().enumerate() {
                output.push_str(&format!("\n  {}. {}", i + 1, ctx));
            }
        }
        
        output
    }

    // Construtores de conveniência
    
    /// Cria um erro de entrada inválida
    #[cfg(feature = "alloc")]
    pub fn invalid_input(message: String) -> Self {
        Self::with_message(1001, ErrorKind::InvalidInput, message)
    }

    /// Cria um erro de recurso não encontrado
    #[cfg(feature = "alloc")]
    pub fn not_found(message: String) -> Self {
        Self::with_message(1002, ErrorKind::NotFound, message)
    }

    /// Cria um erro de permissão negada
    #[cfg(feature = "alloc")]
    pub fn permission_denied(message: String) -> Self {
        Self::with_message(1003, ErrorKind::PermissionDenied, message)
    }

    /// Cria um erro de conexão
    #[cfg(feature = "alloc")]
    pub fn connection_failed(message: String) -> Self {
        Self::with_message(1004, ErrorKind::ConnectionFailed, message)
    }

    /// Cria um erro de timeout
    #[cfg(feature = "alloc")]
    pub fn timeout(message: String) -> Self {
        Self::with_message(1005, ErrorKind::Timeout, message)
    }

    /// Cria um erro de dados corrompidos
    #[cfg(feature = "alloc")]
    pub fn data_corruption(message: String) -> Self {
        Self::with_message(1006, ErrorKind::DataCorruption, message)
    }

    /// Cria um erro de configuração
    #[cfg(feature = "alloc")]
    pub fn config_error(message: String) -> Self {
        Self::with_message(1007, ErrorKind::ConfigError, message)
    }

    /// Cria um erro de autenticação
    #[cfg(feature = "alloc")]
    pub fn authentication_failed(message: String) -> Self {
        Self::with_message(1008, ErrorKind::AuthenticationFailed, message)
    }

    /// Cria um erro de autorização
    #[cfg(feature = "alloc")]
    pub fn authorization_failed(message: String) -> Self {
        Self::with_message(1009, ErrorKind::AuthorizationFailed, message)
    }

    /// Cria um erro de recurso já existente
    #[cfg(feature = "alloc")]
    pub fn already_exists(message: String) -> Self {
        Self::with_message(1010, ErrorKind::AlreadyExists, message)
    }

    /// Cria um erro de recurso esgotado
    #[cfg(feature = "alloc")]
    pub fn resource_exhausted(message: String) -> Self {
        Self::with_message(1011, ErrorKind::ResourceExhausted, message)
    }

    /// Cria um erro de operação cancelada
    #[cfg(feature = "alloc")]
    pub fn cancelled(message: String) -> Self {
        Self::with_message(1012, ErrorKind::Cancelled, message)
    }

    /// Cria um erro interno
    #[cfg(feature = "alloc")]
    pub fn internal(message: String) -> Self {
        Self::with_message(1013, ErrorKind::Internal, message)
    }

    /// Cria um erro de não implementado
    #[cfg(feature = "alloc")]
    pub fn not_implemented(message: String) -> Self {
        Self::with_message(1014, ErrorKind::NotImplemented, message)
    }

    /// Cria um erro de serviço indisponível
    #[cfg(feature = "alloc")]
    pub fn unavailable(message: String) -> Self {
        Self::with_message(1015, ErrorKind::Unavailable, message)
    }

    /// Cria um erro desconhecido
    #[cfg(feature = "alloc")]
    pub fn unknown(message: String) -> Self {
        Self::with_message(9999, ErrorKind::Unknown, message)
    }
}

impl ErrorKind {
    /// Retorna a string representativa do tipo
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidInput => "InvalidInput",
            Self::NotFound => "NotFound",
            Self::PermissionDenied => "PermissionDenied",
            Self::ConnectionFailed => "ConnectionFailed",
            Self::Timeout => "Timeout",
            Self::DataCorruption => "DataCorruption",
            Self::ConfigError => "ConfigError",
            Self::AuthenticationFailed => "AuthenticationFailed",
            Self::AuthorizationFailed => "AuthorizationFailed",
            Self::AlreadyExists => "AlreadyExists",
            Self::ResourceExhausted => "ResourceExhausted",
            Self::Cancelled => "Cancelled",
            Self::Internal => "Internal",
            Self::NotImplemented => "NotImplemented",
            Self::Unavailable => "Unavailable",
            Self::Unknown => "Unknown",
        }
    }

    /// Retorna a mensagem padrão do tipo
    #[inline]
    pub const fn default_message(&self) -> &'static str {
        match self {
            Self::InvalidInput => "Invalid input provided",
            Self::NotFound => "Resource not found",
            Self::PermissionDenied => "Permission denied",
            Self::ConnectionFailed => "Connection failed",
            Self::Timeout => "Operation timed out",
            Self::DataCorruption => "Data corruption detected",
            Self::ConfigError => "Configuration error",
            Self::AuthenticationFailed => "Authentication failed",
            Self::AuthorizationFailed => "Authorization failed",
            Self::AlreadyExists => "Resource already exists",
            Self::ResourceExhausted => "Resource exhausted",
            Self::Cancelled => "Operation cancelled",
            Self::Internal => "Internal error",
            Self::NotImplemented => "Feature not implemented",
            Self::Unavailable => "Service unavailable",
            Self::Unknown => "Unknown error",
        }
    }

    /// Converte código numérico para ErrorKind
    #[inline]
    pub const fn from_u8(code: u8) -> Self {
        match code {
            1 => Self::InvalidInput,
            2 => Self::NotFound,
            3 => Self::PermissionDenied,
            4 => Self::ConnectionFailed,
            5 => Self::Timeout,
            6 => Self::DataCorruption,
            7 => Self::ConfigError,
            8 => Self::AuthenticationFailed,
            9 => Self::AuthorizationFailed,
            10 => Self::AlreadyExists,
            11 => Self::ResourceExhausted,
            12 => Self::Cancelled,
            13 => Self::Internal,
            14 => Self::NotImplemented,
            15 => Self::Unavailable,
            _ => Self::Unknown,
        }
    }
}

// ============================================================================
// Macros
// ============================================================================

/// Retorna antecipadamente com um erro
///
/// # Exemplos
///
/// ```ignore
/// use avila_error::{bail, AvilaError, ErrorKind};
///
/// fn check_value(x: i32) -> Result<(), AvilaError> {
///     if x < 0 {
///         bail!(AvilaError::invalid_input("value must be positive".into()));
///     }
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($err)
    };
}

/// Garante que uma condição seja verdadeira, caso contrário retorna erro
///
/// # Exemplos
///
/// ```ignore
/// use avila_error::{ensure, AvilaError};
///
/// fn check_value(x: i32) -> Result<(), AvilaError> {
///     ensure!(x >= 0, AvilaError::invalid_input("value must be positive".into()));
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !($cond) {
            return Err($err);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = AvilaError::new(1001, ErrorKind::InvalidInput);
        assert_eq!(err.code(), 1001);
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_error_with_message() {
        let err = AvilaError::invalid_input("test error".into());
        assert_eq!(err.code(), 1001);
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.message(), Some("test error"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_error_with_context() {
        let err = AvilaError::not_found("resource missing".into())
            .add_context("in module X".into())
            .add_context("during operation Y".into());
        
        assert_eq!(err.context().len(), 2);
        assert_eq!(err.context()[0], "in module X");
        assert_eq!(err.context()[1], "during operation Y");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_error_format() {
        let err = AvilaError::internal("something went wrong".into())
            .add_context("in function foo".into());
        
        let formatted = err.format();
        assert!(formatted.contains("[1013:Internal]"));
        assert!(formatted.contains("something went wrong"));
        assert!(formatted.contains("in function foo"));
    }

    #[test]
    fn test_error_kind_as_str() {
        assert_eq!(ErrorKind::InvalidInput.as_str(), "InvalidInput");
        assert_eq!(ErrorKind::NotFound.as_str(), "NotFound");
        assert_eq!(ErrorKind::Internal.as_str(), "Internal");
    }

    #[test]
    fn test_error_kind_default_message() {
        assert_eq!(ErrorKind::InvalidInput.default_message(), "Invalid input provided");
        assert_eq!(ErrorKind::NotFound.default_message(), "Resource not found");
    }

    #[test]
    fn test_error_kind_from_u8() {
        assert_eq!(ErrorKind::from_u8(1), ErrorKind::InvalidInput);
        assert_eq!(ErrorKind::from_u8(2), ErrorKind::NotFound);
        assert_eq!(ErrorKind::from_u8(255), ErrorKind::Unknown);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_bail_macro() {
        fn test_fn() -> Result<()> {
            bail!(AvilaError::invalid_input("test".into()));
        }
        
        let result = test_fn();
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_ensure_macro() {
        fn test_fn(x: i32) -> Result<()> {
            ensure!(x > 0, AvilaError::invalid_input("must be positive".into()));
            Ok(())
        }
        
        assert!(test_fn(-1).is_err());
        assert!(test_fn(1).is_ok());
    }

    #[test]
    fn test_all_error_kinds() {
        // Testa todos os 16 tipos de erro
        let kinds = [
            ErrorKind::InvalidInput,
            ErrorKind::NotFound,
            ErrorKind::PermissionDenied,
            ErrorKind::ConnectionFailed,
            ErrorKind::Timeout,
            ErrorKind::DataCorruption,
            ErrorKind::ConfigError,
            ErrorKind::AuthenticationFailed,
            ErrorKind::AuthorizationFailed,
            ErrorKind::AlreadyExists,
            ErrorKind::ResourceExhausted,
            ErrorKind::Cancelled,
            ErrorKind::Internal,
            ErrorKind::NotImplemented,
            ErrorKind::Unavailable,
            ErrorKind::Unknown,
        ];

        for kind in kinds {
            let err = AvilaError::new(1000, kind);
            assert_eq!(err.kind(), kind);
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_all_convenience_constructors() {
        // Testa todos os construtores de conveniência
        let errors = [
            AvilaError::invalid_input("test".into()),
            AvilaError::not_found("test".into()),
            AvilaError::permission_denied("test".into()),
            AvilaError::connection_failed("test".into()),
            AvilaError::timeout("test".into()),
            AvilaError::data_corruption("test".into()),
            AvilaError::config_error("test".into()),
            AvilaError::authentication_failed("test".into()),
            AvilaError::authorization_failed("test".into()),
            AvilaError::already_exists("test".into()),
            AvilaError::resource_exhausted("test".into()),
            AvilaError::cancelled("test".into()),
            AvilaError::internal("test".into()),
            AvilaError::not_implemented("test".into()),
            AvilaError::unavailable("test".into()),
            AvilaError::unknown("test".into()),
        ];

        for err in errors {
            assert_eq!(err.message(), Some("test"));
        }
    }

    #[test]
    fn test_error_clone() {
        let err1 = AvilaError::new(1001, ErrorKind::InvalidInput);
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }
}
