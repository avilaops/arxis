//! POP3 (Post Office Protocol v3) - Recebimento de emails

use avila_error::{Error, ErrorKind, Result};
use avila_molecule::{NetworkAddress, tcp::TcpClient};

/// Cliente POP3
pub struct Pop3Client {
    client: TcpClient,
}

impl Pop3Client {
    /// Conecta a servidor POP3
    pub async fn connect(server: NetworkAddress) -> Result<Self> {
        let client = TcpClient::connect(server).await?;
        Ok(Self { client })
    }

    /// Autenticação
    pub async fn login(&mut self, username: &str, password: &str) -> Result<()> {
        // USER
        let cmd = format!("USER {}\r\n", username);
        self.client.send(cmd.as_bytes()).await?;
        self.expect_ok().await?;

        // PASS
        let cmd = format!("PASS {}\r\n", password);
        self.client.send(cmd.as_bytes()).await?;
        self.expect_ok().await?;

        Ok(())
    }

    /// Lista mensagens
    pub async fn list(&mut self) -> Result<Vec<(u32, usize)>> {
        self.client.send(b"LIST\r\n").await?;
        self.expect_ok().await?;

        // TODO: Parse lista de mensagens
        Ok(Vec::new())
    }

    /// Recupera mensagem
    pub async fn retrieve(&mut self, message_id: u32) -> Result<String> {
        let cmd = format!("RETR {}\r\n", message_id);
        self.client.send(cmd.as_bytes()).await?;

        // TODO: Ler mensagem completa até "."
        Ok(String::new())
    }

    /// Deleta mensagem
    pub async fn delete(&mut self, message_id: u32) -> Result<()> {
        let cmd = format!("DELE {}\r\n", message_id);
        self.client.send(cmd.as_bytes()).await?;
        self.expect_ok().await?;
        Ok(())
    }

    /// Fecha conexão
    pub async fn quit(&mut self) -> Result<()> {
        self.client.send(b"QUIT\r\n").await?;
        Ok(())
    }

    /// Espera resposta +OK
    async fn expect_ok(&mut self) -> Result<String> {
        let mut buffer = vec![0u8; 1024];
        let n = self.client.receive(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]).to_string();

        if !response.starts_with("+OK") {
            return Err(Error::new(
                ErrorKind::Network,
                format!("Resposta POP3 inválida: {}", response),
            ));
        }

        Ok(response)
    }
}
