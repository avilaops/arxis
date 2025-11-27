//! Estrutura de mensagem de email

use crate::EmailAddress;
use avila_error::{Error, ErrorKind, Result};
use avila_time::DateTime;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Mensagem de email completa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    /// ID único da mensagem
    pub id: String,
    /// Remetente
    pub from: EmailAddress,
    /// Destinatários
    pub to: Vec<EmailAddress>,
    /// Cópia
    pub cc: Vec<EmailAddress>,
    /// Cópia oculta
    pub bcc: Vec<EmailAddress>,
    /// Assunto
    pub subject: String,
    /// Corpo da mensagem (texto plano)
    pub body: String,
    /// Corpo HTML (opcional)
    pub html_body: Option<String>,
    /// Headers customizados
    pub headers: HashMap<String, String>,
    /// Data de envio
    pub date: DateTime,
    /// Anexos
    pub attachments: Vec<Attachment>,
}

/// Anexo de email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// Nome do arquivo
    pub filename: String,
    /// Tipo MIME
    pub content_type: String,
    /// Conteúdo (base64 encoded)
    pub content: Vec<u8>,
}

impl Email {
    /// Cria nova mensagem
    pub fn new(from: EmailAddress, to: Vec<EmailAddress>, subject: String, body: String) -> Self {
        Self {
            id: Self::generate_message_id(),
            from,
            to,
            cc: Vec::new(),
            bcc: Vec::new(),
            subject,
            body,
            html_body: None,
            headers: HashMap::new(),
            date: DateTime::now(),
            attachments: Vec::new(),
        }
    }

    /// Gera Message-ID único
    fn generate_message_id() -> String {
        use avila_time::DateTime;
        let timestamp = DateTime::now().timestamp();
        let random: u32 = (timestamp % 1_000_000) as u32;
        format!("<{}.{}@avila.inc>", timestamp, random)
    }

    /// Adiciona destinatário CC
    pub fn add_cc(&mut self, address: EmailAddress) {
        self.cc.push(address);
    }

    /// Adiciona destinatário BCC
    pub fn add_bcc(&mut self, address: EmailAddress) {
        self.bcc.push(address);
    }

    /// Adiciona anexo
    pub fn add_attachment(&mut self, attachment: Attachment) {
        self.attachments.push(attachment);
    }

    /// Define corpo HTML
    pub fn set_html_body(&mut self, html: String) {
        self.html_body = Some(html);
    }

    /// Adiciona header customizado
    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    /// Converte para formato RFC 5322 (email wire format)
    pub fn to_rfc5322(&self) -> String {
        let mut message = String::new();

        // Headers obrigatórios
        message.push_str(&format!("Message-ID: {}\r\n", self.id));
        message.push_str(&format!("From: {}\r\n", self.from));
        message.push_str(&format!("To: {}\r\n",
            self.to.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")));

        if !self.cc.is_empty() {
            message.push_str(&format!("Cc: {}\r\n",
                self.cc.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")));
        }

        message.push_str(&format!("Subject: {}\r\n", self.subject));
        message.push_str(&format!("Date: {}\r\n", self.date.to_rfc2822()));

        // Headers customizados
        for (key, value) in &self.headers {
            message.push_str(&format!("{}: {}\r\n", key, value));
        }

        // Corpo
        message.push_str("\r\n");
        message.push_str(&self.body);

        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_creation() {
        let from = EmailAddress::new("sender@example.com").unwrap();
        let to = vec![EmailAddress::new("recipient@example.com").unwrap()];

        let email = Email::new(from, to, "Test".to_string(), "Hello!".to_string());

        assert_eq!(email.subject, "Test");
        assert_eq!(email.body, "Hello!");
    }

    #[test]
    fn test_rfc5322_format() {
        let from = EmailAddress::new("sender@example.com").unwrap();
        let to = vec![EmailAddress::new("recipient@example.com").unwrap()];

        let email = Email::new(from, to, "Test".to_string(), "Hello!".to_string());
        let rfc = email.to_rfc5322();

        assert!(rfc.contains("From: sender@example.com"));
        assert!(rfc.contains("To: recipient@example.com"));
        assert!(rfc.contains("Subject: Test"));
        assert!(rfc.contains("Hello!"));
    }
}
