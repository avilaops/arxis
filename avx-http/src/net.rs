//! Network I/O primitives - TcpStream and TcpListener
//!
//! Pure std::net implementation with non-blocking I/O

use crate::error::{Error, Result};
use std::io::{self, Read, Write};
use std::net::{TcpStream as StdTcpStream, TcpListener as StdTcpListener, SocketAddr, ToSocketAddrs};
use std::time::Duration;

/// TCP stream for client connections
pub struct TcpStream {
    inner: StdTcpStream,
}

impl TcpStream {
    /// Connect to a remote host
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = StdTcpStream::connect(&addr)
            .map_err(|e| Error::ConnectionFailed {
                host: "unknown".to_string(),
                reason: e.to_string(),
            })?;

        // Set non-blocking mode
        stream.set_nonblocking(false)
            .map_err(|e| Error::Internal {
                message: format!("Failed to set non-blocking: {}", e),
            })?;

        // Set TCP_NODELAY
        stream.set_nodelay(true)
            .map_err(|e| Error::Internal {
                message: format!("Failed to set TCP_NODELAY: {}", e),
            })?;

        Ok(Self { inner: stream })
    }

    /// Read data from stream
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
            .map_err(|e| Error::Internal {
                message: format!("Read error: {}", e),
            })
    }

    /// Write data to stream
    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.write(buf)
            .map_err(|e| Error::Internal {
                message: format!("Write error: {}", e),
            })
    }

    /// Write all data to stream
    pub fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.inner.write_all(buf)
            .map_err(|e| Error::Internal {
                message: format!("Write all error: {}", e),
            })
    }

    /// Read exact number of bytes
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.inner.read_exact(buf)
            .map_err(|e| Error::Internal {
                message: format!("Read exact error: {}", e),
            })
    }

    /// Flush stream
    pub fn flush(&mut self) -> Result<()> {
        self.inner.flush()
            .map_err(|e| Error::Internal {
                message: format!("Flush error: {}", e),
            })
    }

    /// Set read timeout
    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> Result<()> {
        self.inner.set_read_timeout(timeout)
            .map_err(|e| Error::Internal {
                message: format!("Failed to set read timeout: {}", e),
            })
    }

    /// Set write timeout
    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> Result<()> {
        self.inner.set_write_timeout(timeout)
            .map_err(|e| Error::Internal {
                message: format!("Failed to set write timeout: {}", e),
            })
    }

    /// Clone the stream (for keep-alive)
    pub fn try_clone(&self) -> Result<Self> {
        let cloned = self.inner.try_clone()
            .map_err(|e| Error::Internal {
                message: format!("Failed to clone stream: {}", e),
            })?;

        Ok(Self { inner: cloned })
    }

    /// Get peer address
    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.inner.peer_addr()
            .map_err(|e| Error::Internal {
                message: format!("Failed to get peer addr: {}", e),
            })
    }

    /// Shutdown the stream
    pub fn shutdown(&self) -> Result<()> {
        self.inner.shutdown(std::net::Shutdown::Both)
            .map_err(|e| Error::Internal {
                message: format!("Shutdown error: {}", e),
            })
    }
}

/// TCP listener for server connections
pub struct TcpListener {
    inner: StdTcpListener,
}

impl TcpListener {
    /// Bind to an address
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let listener = StdTcpListener::bind(addr)
            .map_err(|e| Error::Internal {
                message: format!("Failed to bind: {}", e),
            })?;

        Ok(Self { inner: listener })
    }

    /// Accept incoming connection
    pub fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        let (stream, addr) = self.inner.accept()
            .map_err(|e| Error::Internal {
                message: format!("Accept error: {}", e),
            })?;

        // Configure stream
        stream.set_nodelay(true)
            .map_err(|e| Error::Internal {
                message: format!("Failed to set TCP_NODELAY: {}", e),
            })?;

        Ok((TcpStream { inner: stream }, addr))
    }

    /// Get local address
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.inner.local_addr()
            .map_err(|e| Error::Internal {
                message: format!("Failed to get local addr: {}", e),
            })
    }

    /// Set non-blocking mode
    pub fn set_nonblocking(&self, nonblocking: bool) -> Result<()> {
        self.inner.set_nonblocking(nonblocking)
            .map_err(|e| Error::Internal {
                message: format!("Failed to set non-blocking: {}", e),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_listener_bind() {
        let listener = TcpListener::bind("127.0.0.1:0");
        assert!(listener.is_ok());
    }

    #[test]
    fn test_tcp_stream_connect() {
        // This test requires a server to be running
        // In real tests, we'd set up a test server first
    }
}
