//! SMTP (Simple Mail Transfer Protocol) - Envio de emails

use crate::{EmailAddress, message::Email};
use avila_error::{Error, ErrorKind, Result};
use avila_molecule::{NetworkAddress, tcp::TcpClient};
use tokio::io::{AsyncBufReadExt, BufReader};

/// Cliente SMTP
pub struct SmtpClient {
    client: TcpClient,
    server: NetworkAddress,
}

impl SmtpClient {
    /// Conecta a servidor SMTP
    pub async fn connect(server: NetworkAddress) -> Result<Self> {
        let mut client = TcpClient::connect(server.clone()).await?;

        // Lê banner do servidor
        let mut buffer = vec![0u8; 1024];
        let n = client.receive(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]);

        if !response.starts_with("220") {
            return Err(Error::new(
                ErrorKind::Network,
                format!("Banner SMTP inválido: {}", response),
            ));
        }

        Ok(Self { client, server })
    }

    /// Envia comando HELO
    pub async fn helo(&mut self, domain: &str) -> Result<()> {
        let cmd = format!("HELO {}\r\n", domain);
        self.client.send(cmd.as_bytes()).await?;

        let mut buffer = vec![0u8; 1024];
        let n = self.client.receive(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]);

        if !response.starts_with("250") {
            return Err(Error::new(
                ErrorKind::Network,
                format!("HELO falhou: {}", response),
            ));
        }

        Ok(())
    }

    /// Envia email
    pub async fn send_email(&mut self, email: &Email) -> Result<()> {
        // MAIL FROM
        let cmd = format!("MAIL FROM:<{}>\r\n", email.from);
        self.client.send(cmd.as_bytes()).await?;
        self.expect_response("250").await?;

        // RCPT TO (para cada destinatário)
        for recipient in &email.to {
            let cmd = format!("RCPT TO:<{}>\r\n", recipient);
            self.client.send(cmd.as_bytes()).await?;
            self.expect_response("250").await?;
        }

        // DATA
        self.client.send(b"DATA\r\n").await?;
        self.expect_response("354").await?;

        // Corpo da mensagem
        let message = email.to_rfc5322();
        self.client.send(message.as_bytes()).await?;
        self.client.send(b"\r\n.\r\n").await?; // End of data
        self.expect_response("250").await?;

        Ok(())
    }

    /// Fecha conexão
    pub async fn quit(&mut self) -> Result<()> {
        self.client.send(b"QUIT\r\n").await?;
        Ok(())
    }

    /// Espera resposta com código específico
    async fn expect_response(&mut self, expected_code: &str) -> Result<String> {
        let mut buffer = vec![0u8; 1024];
        let n = self.client.receive(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]).to_string();

        if !response.starts_with(expected_code) {
            return Err(Error::new(
                ErrorKind::Network,
                format!("Resposta inesperada: esperado {}, recebido {}", expected_code, response),
            ));
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requer servidor SMTP rodando
    async fn test_smtp_connection() {
        let addr = NetworkAddress::new("localhost", 2525);
        let result = SmtpClient::connect(addr).await;
        // Teste depende de servidor local
    }
}
