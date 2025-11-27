//! TLS support using rustls
//!
//! This module provides TLS 1.3 support through rustls integration.
//! Enabled with the `tls` feature flag.

#[cfg(feature = "tls")]
use rustls::{ClientConfig, ServerConfig, ClientConnection, ServerConnection};
#[cfg(feature = "tls")]
use std::sync::Arc;
#[cfg(feature = "tls")]
use std::io::{Read, Write};

use crate::error::{Error, Result};
use crate::async_net::AsyncTcpStream;

/// TLS client connector
#[cfg(feature = "tls")]
pub struct TlsConnector {
    config: Arc<ClientConfig>,
}

#[cfg(feature = "tls")]
impl TlsConnector {
    /// Create new TLS connector with default configuration
    pub fn new() -> Result<Self> {
        let mut root_store = rustls::RootCertStore::empty();

        // Add system root certificates
        for cert in rustls_native_certs::load_native_certs().expect("Failed to load native certs") {
            root_store.add(cert).ok();
        }

        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        Ok(Self {
            config: Arc::new(config),
        })
    }

    /// Connect to server with TLS
    pub async fn connect(
        &self,
        domain: &str,
        stream: AsyncTcpStream,
    ) -> Result<TlsStream> {
        let domain_owned = domain.to_string();
        let server_name = rustls::pki_types::ServerName::try_from(domain_owned.as_str())
            .map_err(|e| Error::Internal {
                message: format!("Invalid server name: {:?}", e),
            })?
            .to_owned();

        let conn = ClientConnection::new(Arc::clone(&self.config), server_name)
            .map_err(|e| Error::Internal {
                message: format!("TLS connection failed: {}", e),
            })?;

        Ok(TlsStream {
            stream,
            conn: TlsConnection::Client(conn),
            read_buf: Vec::new(),
            write_buf: Vec::new(),
        })
    }
}

#[cfg(feature = "tls")]
impl Default for TlsConnector {
    fn default() -> Self {
        Self::new().expect("Failed to create default TLS connector")
    }
}

/// TLS connection wrapper
#[cfg(feature = "tls")]
enum TlsConnection {
    Client(ClientConnection),
    Server(ServerConnection),
}

/// TLS stream
#[cfg(feature = "tls")]
pub struct TlsStream {
    stream: AsyncTcpStream,
    conn: TlsConnection,
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,
}

#[cfg(feature = "tls")]
impl TlsStream {
    /// Read data from TLS stream
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // Process TLS handshake if needed
        self.process_tls().await?;

        // Read decrypted data
        if !self.read_buf.is_empty() {
            let n = std::cmp::min(buf.len(), self.read_buf.len());
            buf[..n].copy_from_slice(&self.read_buf[..n]);
            self.read_buf.drain(..n);
            return Ok(n);
        }

        // Read more data from network
        let mut net_buf = vec![0u8; 16384];
        let n = self.stream.read(&mut net_buf).await?;
        if n == 0 {
            return Ok(0);
        }

        // Feed to rustls
        match &mut self.conn {
            TlsConnection::Client(conn) => {
                conn.read_tls(&mut &net_buf[..n])
                    .map_err(|e| Error::Internal {
                        message: format!("TLS read error: {}", e),
                    })?;

                conn.process_new_packets()
                    .map_err(|e| Error::Internal {
                        message: format!("TLS process error: {}", e),
                    })?;

                // Read plaintext
                let mut plaintext = Vec::new();
                match conn.reader().read_to_end(&mut plaintext) {
                    Ok(_) => {
                        let n = std::cmp::min(buf.len(), plaintext.len());
                        buf[..n].copy_from_slice(&plaintext[..n]);
                        if plaintext.len() > n {
                            self.read_buf.extend_from_slice(&plaintext[n..]);
                        }
                        Ok(n)
                    }
                    Err(e) => Err(Error::Internal {
                        message: format!("Failed to read plaintext: {}", e),
                    }),
                }
            }
            TlsConnection::Server(_) => {
                // Similar logic for server
                Err(Error::Internal {
                    message: "Server TLS not yet implemented".to_string(),
                })
            }
        }
    }

    /// Write data to TLS stream
    pub async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.process_tls().await?;

        // Write plaintext to rustls
        let n = match &mut self.conn {
            TlsConnection::Client(conn) => {
                conn.writer().write(buf)
                    .map_err(|e| Error::Internal {
                        message: format!("TLS write error: {}", e),
                    })?
            }
            TlsConnection::Server(_) => {
                return Err(Error::Internal {
                    message: "Server TLS not yet implemented".to_string(),
                });
            }
        };

        // Flush encrypted data to network
        self.flush_tls().await?;

        Ok(n)
    }

    /// Write all data
    pub async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        let mut written = 0;
        while written < buf.len() {
            let n = self.write(&buf[written..]).await?;
            written += n;
        }
        Ok(())
    }

    /// Process TLS handshake and encrypted data
    async fn process_tls(&mut self) -> Result<()> {
        match &mut self.conn {
            TlsConnection::Client(conn) => {
                // Write any pending data
                while conn.wants_write() {
                    self.flush_tls().await?;
                }

                // Read if needed
                if conn.wants_read() {
                    let mut buf = vec![0u8; 16384];
                    let n = self.stream.read(&mut buf).await?;
                    if n > 0 {
                        conn.read_tls(&mut &buf[..n])
                            .map_err(|e| Error::Internal {
                                message: format!("TLS read error: {}", e),
                            })?;

                        conn.process_new_packets()
                            .map_err(|e| Error::Internal {
                                message: format!("TLS process error: {}", e),
                            })?;
                    }
                }
            }
            TlsConnection::Server(_) => {
                // Server logic
            }
        }

        Ok(())
    }

    /// Flush encrypted data to network
    async fn flush_tls(&mut self) -> Result<()> {
        match &mut self.conn {
            TlsConnection::Client(conn) => {
                let mut buf = Vec::new();
                conn.write_tls(&mut buf)
                    .map_err(|e| Error::Internal {
                        message: format!("TLS write error: {}", e),
                    })?;

                if !buf.is_empty() {
                    self.stream.write_all(&buf).await?;
                }
            }
            TlsConnection::Server(_) => {}
        }

        Ok(())
    }
}

// Stub implementations when TLS is disabled
#[cfg(not(feature = "tls"))]
pub struct TlsConnector;

#[cfg(not(feature = "tls"))]
impl TlsConnector {
    pub fn new() -> Result<Self> {
        Err(Error::Internal {
            message: "TLS support not enabled. Enable 'tls' feature.".to_string(),
        })
    }
}

#[cfg(not(feature = "tls"))]
pub struct TlsStream;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "tls")]
    fn test_tls_connector_creation() {
        let connector = TlsConnector::new();
        assert!(connector.is_ok());
    }

    #[test]
    #[cfg(not(feature = "tls"))]
    fn test_tls_disabled() {
        let result = TlsConnector::new();
        assert!(result.is_err());
    }
}
