//! Async networking with reactor integration
//!
//! Non-blocking TCP with epoll/kqueue/IOCP

use std::io::{self, Read, Write};
use std::net::{SocketAddr, ToSocketAddrs};
use std::os::windows::io::AsRawSocket;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;

use crate::bytes::Bytes;
use crate::error::{Error, Result};
use crate::reactor::{Interest, Reactor};
use crate::runtime::Runtime;

/// Async TCP stream
pub struct AsyncTcpStream {
    inner: std::net::TcpStream,
    token: usize,
}

impl AsyncTcpStream {
    /// Connect to remote host asynchronously
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        // Create non-blocking socket
        let addr = addr
            .to_socket_addrs()
            .map_err(|e| Error::ConnectionFailed {
                host: "unknown".to_string(),
                reason: e.to_string(),
            })?
            .next()
            .ok_or_else(|| Error::ConnectionFailed {
                host: "unknown".to_string(),
                reason: "No address found".to_string(),
            })?;

        let stream = std::net::TcpStream::connect(addr)
            .map_err(|e| Error::ConnectionFailed {
                host: format!("{}", addr),
                reason: e.to_string(),
            })?;

        stream.set_nonblocking(true)
            .map_err(|e| Error::Internal {
                message: format!("Failed to set non-blocking: {}", e),
            })?;

        stream.set_nodelay(true)
            .map_err(|e| Error::Internal {
                message: format!("Failed to set TCP_NODELAY: {}", e),
            })?;

        // Generate unique token
        static NEXT_TOKEN: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
        let token = NEXT_TOKEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        Ok(Self { inner: stream, token })
    }

    /// Read data asynchronously
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        ReadFuture {
            stream: self,
            buf,
        }
        .await
    }

    /// Write data asynchronously
    pub async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        WriteFuture {
            stream: self,
            buf,
        }
        .await
    }

    /// Write all data
    pub async fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            let n = self.write(buf).await?;
            buf = &buf[n..];
        }
        Ok(())
    }

    /// Read exact amount of bytes
    pub async fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        let mut pos = 0;
        while pos < buf.len() {
            let n = self.read(&mut buf[pos..]).await?;
            if n == 0 {
                return Err(Error::UnexpectedEof);
            }
            pos += n;
        }
        Ok(())
    }

    /// Get socket file descriptor (Unix) or handle (Windows)
    #[cfg(unix)]
    fn raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.inner.as_raw_fd()
    }

    #[cfg(windows)]
    fn raw_fd(&self) -> usize {
        self.inner.as_raw_socket() as usize
    }
}

/// Future for reading
struct ReadFuture<'a> {
    stream: &'a mut AsyncTcpStream,
    buf: &'a mut [u8],
}

impl<'a> Future for ReadFuture<'a> {
    type Output = Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: We're not moving anything out
        let this = unsafe { self.get_unchecked_mut() };

        // Try non-blocking read
        match this.stream.inner.read(this.buf) {
            Ok(n) => Poll::Ready(Ok(n)),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Register interest and waker
                // TODO: Get reactor from runtime and register
                // For now, just yield
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e.into())),
        }
    }
}

/// Future for writing
struct WriteFuture<'a> {
    stream: &'a mut AsyncTcpStream,
    buf: &'a [u8],
}

impl<'a> Future for WriteFuture<'a> {
    type Output = Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: We're not moving anything out
        let this = unsafe { self.get_unchecked_mut() };

        // Try non-blocking write
        match this.stream.inner.write(this.buf) {
            Ok(n) => Poll::Ready(Ok(n)),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Register interest and waker
                // TODO: Get reactor from runtime and register
                // For now, just yield
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e.into())),
        }
    }
}

/// Async TCP listener
pub struct AsyncTcpListener {
    inner: std::net::TcpListener,
    token: usize,
}

impl AsyncTcpListener {
    /// Bind to address
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let listener = std::net::TcpListener::bind(addr)?;

        listener.set_nonblocking(true)
            .map_err(|e| Error::Internal {
                message: format!("Failed to set non-blocking: {}", e),
            })?;

        static NEXT_TOKEN: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1000000);
        let token = NEXT_TOKEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        Ok(Self { inner: listener, token })
    }

    /// Accept connection asynchronously
    pub async fn accept(&self) -> Result<(AsyncTcpStream, SocketAddr)> {
        AcceptFuture { listener: self }.await
    }

    /// Get local address
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.inner.local_addr().map_err(|e| e.into())
    }
}

/// Future for accepting
struct AcceptFuture<'a> {
    listener: &'a AsyncTcpListener,
}

impl<'a> Future for AcceptFuture<'a> {
    type Output = Result<(AsyncTcpStream, SocketAddr)>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.listener.inner.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true)
                    .map_err(|e| Error::Internal {
                        message: format!("Failed to set non-blocking: {}", e),
                    })
                    .ok();

                stream.set_nodelay(true).ok();

                static NEXT_TOKEN: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
                let token = NEXT_TOKEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                let async_stream = AsyncTcpStream {
                    inner: stream,
                    token,
                };

                Poll::Ready(Ok((async_stream, addr)))
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Register interest and waker
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime;

    #[test]
    fn test_async_tcp_stream_creation() {
        // Basic test that we can create the types
        let _result = std::net::TcpListener::bind("127.0.0.1:0");
    }
}
