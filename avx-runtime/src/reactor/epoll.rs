//! # Epoll - Linux native event loop
//!
//! Implementação usando syscalls diretos:
//! - epoll_create1
//! - epoll_ctl
//! - epoll_wait

use std::io;
use std::os::unix::io::RawFd;
use std::time::Duration;
use crate::reactor::Interest;

const EPOLL_CTL_ADD: i32 = 1;
const EPOLL_CTL_DEL: i32 = 2;
const EPOLL_CTL_MOD: i32 = 3;

const EPOLLIN: u32 = 0x001;
const EPOLLOUT: u32 = 0x004;
const EPOLLET: u32 = 1 << 31;
const EPOLLONESHOT: u32 = 1 << 30;

const EPOLL_CLOEXEC: i32 = 0x80000;

#[repr(C)]
#[derive(Clone, Copy)]
struct EpollEvent {
    events: u32,
    data: u64,
}

pub struct EpollReactor {
    epoll_fd: RawFd,
    waker_fd: RawFd,
}

impl EpollReactor {
    pub fn new() -> io::Result<Self> {
        // Cria epoll instance
        let epoll_fd = unsafe {
            syscall!(epoll_create1(EPOLL_CLOEXEC))
        };

        if epoll_fd < 0 {
            return Err(io::Error::last_os_error());
        }

        // Cria eventfd para wakeup
        let waker_fd = unsafe {
            syscall!(eventfd(0, EFD_CLOEXEC | EFD_NONBLOCK))
        };

        if waker_fd < 0 {
            unsafe { syscall!(close(epoll_fd)) };
            return Err(io::Error::last_os_error());
        }

        let reactor = Self { epoll_fd, waker_fd };

        // Registra waker_fd no epoll
        let mut event = EpollEvent {
            events: EPOLLIN | EPOLLET,
            data: usize::MAX as u64, // Token especial para waker
        };

        let ret = unsafe {
            syscall!(epoll_ctl(
                reactor.epoll_fd,
                EPOLL_CTL_ADD,
                reactor.waker_fd,
                &mut event as *mut EpollEvent
            ))
        };

        if ret < 0 {
            unsafe {
                syscall!(close(epoll_fd));
                syscall!(close(waker_fd));
            }
            return Err(io::Error::last_os_error());
        }

        Ok(reactor)
    }

    pub fn register(&self, fd: i32, token: usize, interest: Interest) -> io::Result<()> {
        let mut events = EPOLLET; // Edge-triggered

        if interest.readable {
            events |= EPOLLIN;
        }
        if interest.writable {
            events |= EPOLLOUT;
        }

        let mut event = EpollEvent {
            events,
            data: token as u64,
        };

        let ret = unsafe {
            syscall!(epoll_ctl(
                self.epoll_fd,
                EPOLL_CTL_ADD,
                fd,
                &mut event as *mut EpollEvent
            ))
        };

        if ret < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn deregister(&self, fd: i32) -> io::Result<()> {
        let ret = unsafe {
            syscall!(epoll_ctl(
                self.epoll_fd,
                EPOLL_CTL_DEL,
                fd,
                std::ptr::null_mut()
            ))
        };

        if ret < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn wake(&self) {
        // Escreve no eventfd para acordar epoll_wait
        let value: u64 = 1;
        unsafe {
            syscall!(write(
                self.waker_fd,
                &value as *const u64 as *const libc::c_void,
                8
            ));
        }
    }

    pub fn wait(&self, timeout: Option<Duration>) -> io::Result<usize> {
        const MAX_EVENTS: usize = 1024;
        let mut events: [EpollEvent; MAX_EVENTS] = unsafe { std::mem::zeroed() };

        let timeout_ms = timeout
            .map(|d| d.as_millis() as i32)
            .unwrap_or(-1); // -1 = block indefinitely

        let n = unsafe {
            syscall!(epoll_wait(
                self.epoll_fd,
                events.as_mut_ptr(),
                MAX_EVENTS as i32,
                timeout_ms
            ))
        };

        if n < 0 {
            return Err(io::Error::last_os_error());
        }

        // Limpa waker se foi acionado
        for i in 0..(n as usize) {
            if events[i].data == usize::MAX as u64 {
                let mut buf: u64 = 0;
                unsafe {
                    syscall!(read(
                        self.waker_fd,
                        &mut buf as *mut u64 as *mut libc::c_void,
                        8
                    ));
                }
            }
        }

        Ok(n as usize)
    }
}

impl Drop for EpollReactor {
    fn drop(&mut self) {
        unsafe {
            syscall!(close(self.waker_fd));
            syscall!(close(self.epoll_fd));
        }
    }
}

// Syscall wrappers - implementação direta sem libc
#[cfg(target_arch = "x86_64")]
const SYS_EPOLL_CREATE1: i64 = 291;
#[cfg(target_arch = "x86_64")]
const SYS_EPOLL_CTL: i64 = 233;
#[cfg(target_arch = "x86_64")]
const SYS_EPOLL_WAIT: i64 = 232;
#[cfg(target_arch = "x86_64")]
const SYS_EVENTFD: i64 = 284;
#[cfg(target_arch = "x86_64")]
const SYS_READ: i64 = 0;
#[cfg(target_arch = "x86_64")]
const SYS_WRITE: i64 = 1;
#[cfg(target_arch = "x86_64")]
const SYS_CLOSE: i64 = 3;

macro_rules! syscall {
    ($fn:ident($($arg:expr),* $(,)?)) => {{
        let res = unsafe {
            match stringify!($fn) {
                "epoll_create1" => syscall1(SYS_EPOLL_CREATE1, $($arg),*),
                "epoll_ctl" => syscall4(SYS_EPOLL_CTL, $($arg as i64),*),
                "epoll_wait" => syscall4(SYS_EPOLL_WAIT, $($arg as i64),*),
                "eventfd" => syscall2(SYS_EVENTFD, $($arg as i64),*),
                "read" => syscall3(SYS_READ, $($arg as i64),*),
                "write" => syscall3(SYS_WRITE, $($arg as i64),*),
                "close" => syscall1(SYS_CLOSE, $($arg as i64),*),
                _ => -1,
            }
        };
        res as isize
    }};
}

#[cfg(target_arch = "x86_64")]
unsafe fn syscall1(n: i64, a1: i64) -> i64 {
    let ret: i64;
    std::arch::asm!(
        "syscall",
        inlateout("rax") n => ret,
        in("rdi") a1,
        lateout("rcx") _,
        lateout("r11") _,
    );
    ret
}

#[cfg(target_arch = "x86_64")]
unsafe fn syscall2(n: i64, a1: i64, a2: i64) -> i64 {
    let ret: i64;
    std::arch::asm!(
        "syscall",
        inlateout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        lateout("rcx") _,
        lateout("r11") _,
    );
    ret
}

#[cfg(target_arch = "x86_64")]
unsafe fn syscall3(n: i64, a1: i64, a2: i64, a3: i64) -> i64 {
    let ret: i64;
    std::arch::asm!(
        "syscall",
        inlateout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        lateout("rcx") _,
        lateout("r11") _,
    );
    ret
}

#[cfg(target_arch = "x86_64")]
unsafe fn syscall4(n: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64 {
    let ret: i64;
    std::arch::asm!(
        "syscall",
        inlateout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        lateout("rcx") _,
        lateout("r11") _,
    );
    ret
}

const EFD_CLOEXEC: i32 = 0x80000;
const EFD_NONBLOCK: i32 = 0x800;

unsafe impl Send for EpollReactor {}
unsafe impl Sync for EpollReactor {}
