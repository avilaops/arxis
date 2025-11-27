//! # Reactor - Event Loop baseado em epoll/kqueue/IOCP
//!
//! Implementação nativa sem dependências:
//! - Linux: epoll syscalls
//! - macOS/BSD: kqueue syscalls
//! - Windows: IOCP WinAPI

use std::io;
use std::time::Duration;

#[cfg(target_os = "linux")]
mod epoll;

#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
mod kqueue;

#[cfg(target_os = "windows")]
mod iocp;

/// Event loop nativo multi-plataforma.
///
/// Abstrai diferenças entre epoll (Linux), kqueue (BSD/macOS) e IOCP (Windows).
pub struct Reactor {
    #[cfg(target_os = "linux")]
    inner: epoll::EpollReactor,

    #[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
    inner: kqueue::KqueueReactor,

    #[cfg(target_os = "windows")]
    inner: iocp::IocpReactor,
}

impl Reactor {
    /// Cria novo reactor para a plataforma atual.
    pub fn new() -> io::Result<Self> {
        #[cfg(target_os = "linux")]
        let inner = epoll::EpollReactor::new()?;

        #[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
        let inner = kqueue::KqueueReactor::new()?;

        #[cfg(target_os = "windows")]
        let inner = iocp::IocpReactor::new()?;

        Ok(Self { inner })
    }

    /// Acorda o reactor, interrompendo wait() em andamento.
    pub fn wake(&self) {
        self.inner.wake();
    }

    /// Aguarda eventos de I/O até timeout.
    ///
    /// # Retorna
    ///
    /// Número de eventos processados.
    pub fn wait(&self, timeout: Option<Duration>) -> io::Result<usize> {
        self.inner.wait(timeout)
    }

    /// Registra file descriptor para monitoramento de I/O.
    ///
    /// # Argumentos
    ///
    /// * `fd` - File descriptor
    /// * `token` - Token identificador do evento
    /// * `interest` - Tipo de interesse (leitura/escrita)
    pub fn register(&self, fd: i32, token: usize, interest: Interest) -> io::Result<()> {
        self.inner.register(fd, token, interest)
    }

    /// Remove file descriptor do monitoramento.
    pub fn deregister(&self, fd: i32) -> io::Result<()> {
        self.inner.deregister(fd)
    }
}

/// Tipo de interesse em operações de I/O.
///
/// Especifica se deseja monitorar leitura, escrita ou ambos.
#[derive(Copy, Clone, Debug)]
pub struct Interest {
    /// Interesse em leitura
    pub readable: bool,
    /// Interesse em escrita
    pub writable: bool,
}

impl Interest {
    /// Cria interesse somente em leitura.
    pub fn readable() -> Self {
        Self { readable: true, writable: false }
    }

    /// Cria interesse somente em escrita.
    pub fn writable() -> Self {
        Self { readable: false, writable: true }
    }

    /// Cria interesse em leitura E escrita.
    pub fn both() -> Self {
        Self { readable: true, writable: true }
    }
}
