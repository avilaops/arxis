//! TLS/SSL Termination Module
//!
//! Provides HTTPS support with rustls for the load balancer.

use anyhow::{Context, Result};
use rustls::ServerConfig;
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use tokio_rustls::TlsAcceptor;

/// TLS configuration for the load balancer
#[derive(Clone)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
    pub acceptor: TlsAcceptor,
}

impl TlsConfig {
    /// Load TLS configuration from certificate and key files
    pub fn from_pem_files<P: AsRef<Path>>(cert_path: P, key_path: P) -> Result<Self> {
        let cert_path_str = cert_path.as_ref().to_string_lossy().to_string();
        let key_path_str = key_path.as_ref().to_string_lossy().to_string();

        // Load certificates
        let cert_file = File::open(&cert_path)
            .with_context(|| format!("Failed to open certificate file: {}", cert_path_str))?;
        let mut cert_reader = BufReader::new(cert_file);
        let cert_chain: Vec<_> = certs(&mut cert_reader)
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse certificate chain")?;

        if cert_chain.is_empty() {
            anyhow::bail!("No certificates found in {}", cert_path_str);
        }

        // Load private key
        let key_file = File::open(&key_path)
            .with_context(|| format!("Failed to open private key file: {}", key_path_str))?;
        let mut key_reader = BufReader::new(key_file);
        let mut keys = pkcs8_private_keys(&mut key_reader)
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse private key")?;

        if keys.is_empty() {
            anyhow::bail!("No private key found in {}", key_path_str);
        }

        let key = keys.remove(0);

        // Build TLS config
        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key.into())
            .context("Failed to build TLS configuration")?;

        let acceptor = TlsAcceptor::from(Arc::new(config));

        Ok(TlsConfig {
            cert_path: cert_path_str,
            key_path: key_path_str,
            acceptor,
        })
    }

    /// Get the TLS acceptor
    pub fn acceptor(&self) -> &TlsAcceptor {
        &self.acceptor
    }

    /// Reload TLS certificates (useful for certificate rotation)
    pub fn reload(&mut self) -> Result<()> {
        let new_config = Self::from_pem_files(&self.cert_path, &self.key_path)?;
        self.acceptor = new_config.acceptor;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_config_missing_files() {
        let result = TlsConfig::from_pem_files("nonexistent.crt", "nonexistent.key");
        assert!(result.is_err());
    }
}
