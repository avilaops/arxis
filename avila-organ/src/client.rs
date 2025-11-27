//! Cliente de email

use avila_error::Result;
use avila_cell::smtp::SmtpClient;
use avila_cell::message::Email;

pub struct EmailClient {
    smtp_client: Option<SmtpClient>,
}

impl EmailClient {
    pub fn new() -> Self {
        Self { smtp_client: None }
    }

    pub async fn send(&mut self, email: &Email) -> Result<()> {
        // TODO: Implementar envio
        Ok(())
    }
}
