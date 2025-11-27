//! # Kqueue - macOS/BSD native event loop
//!
//! Implementação usando syscalls diretos:
//! - kqueue
//! - kevent

use std::io;
use std::os::unix::io::RawFd;
use std::time::Duration;
use crate::reactor::Interest;

const EVFILT_READ: i16 = -1;
const EVFILT_WRITE: i16 = -2;
const EVFILT_USER: i16 = -10;

const EV_ADD: u16 = 0x0001;
const EV_DELETE: u16 = 0x0002;
const EV_ENABLE: u16 = 0x0004;
const EV_CLEAR: u16 = 0x0020;
const EV_ONESHOT: u16 = 0x0010;

#[cfg(target_os = "macos")]
const SYS_KQUEUE: i64 = 362;
#[cfg(target_os = "macos")]
const SYS_KEVENT: i64 = 363;
#[cfg(target_os = "macos")]
const SYS_CLOSE: i64 = 6;

#[repr(C)]
struct Kevent {
    ident: usize,
    filter: i16,
    flags: u16,
    fflags: u32,
    data: isize,
    udata: *mut libc::c_void,
}

#[repr(C)]
struct Timespec {
    tv_sec: isize,
    tv_nsec: isize,
}

pub struct KqueueReactor {
    kqueue_fd: RawFd,
}

impl KqueueReactor {
    pub fn new() -> io::Result<Self> {
        #[cfg(target_os = "macos")]
        let kqueue_fd = unsafe {
            syscall0(SYS_KQUEUE) as i32
        };

        #[cfg(not(target_os = "macos"))]
        let kqueue_fd = -1; // Fallback for non-macOS

        if kqueue_fd < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self { kqueue_fd })
    }

    pub fn register(&self, fd: i32, token: usize, interest: Interest) -> io::Result<()> {
        let mut changes = Vec::new();

        if interest.readable {
            changes.push(Kevent {
                ident: fd as usize,
                filter: EVFILT_READ,
                flags: EV_ADD | EV_ENABLE | EV_CLEAR,
                fflags: 0,
                data: 0,
                udata: token as *mut libc::c_void,
            });
        }

        if interest.writable {
            changes.push(Kevent {
                ident: fd as usize,
                filter: EVFILT_WRITE,
                flags: EV_ADD | EV_ENABLE | EV_CLEAR,
                fflags: 0,
                data: 0,
                udata: token as *mut libc::c_void,
            });
        }

        let ret = unsafe {
            kevent_syscall(
                self.kqueue_fd,
                changes.as_ptr(),
                changes.len() as i32,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
            )
        };

        if ret < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn deregister(&self, fd: i32) -> io::Result<()> {
        let changes = [
            Kevent {
                ident: fd as usize,
                filter: EVFILT_READ,
                flags: EV_DELETE,
                fflags: 0,
                data: 0,
                udata: std::ptr::null_mut(),
            },
            Kevent {
                ident: fd as usize,
                filter: EVFILT_WRITE,
                flags: EV_DELETE,
                fflags: 0,
                data: 0,
                udata: std::ptr::null_mut(),
            },
        ];

        unsafe {
            kevent_syscall(
                self.kqueue_fd,
                changes.as_ptr(),
                changes.len() as i32,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
            );
        }

        Ok(())
    }

    pub fn wake(&self) {
        // Trigger user event para wakeup
        let event = Kevent {
            ident: 0,
            filter: EVFILT_USER,
            flags: EV_ADD | EV_ENABLE | EV_ONESHOT,
            fflags: 1, // NOTE_TRIGGER
            data: 0,
            udata: std::ptr::null_mut(),
        };

        unsafe {
            kevent_syscall(
                self.kqueue_fd,
                &event as *const Kevent,
                1,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
            );
        }
    }

    pub fn wait(&self, timeout: Option<Duration>) -> io::Result<usize> {
        const MAX_EVENTS: usize = 1024;
        let mut events: [Kevent; MAX_EVENTS] = unsafe { std::mem::zeroed() };

        let timeout_ptr = if let Some(duration) = timeout {
            let ts = Timespec {
                tv_sec: duration.as_secs() as isize,
                tv_nsec: duration.subsec_nanos() as isize,
            };
            &ts as *const Timespec
        } else {
            std::ptr::null()
        };

        let n = unsafe {
            kevent_syscall(
                self.kqueue_fd,
                std::ptr::null(),
                0,
                events.as_mut_ptr(),
                MAX_EVENTS as i32,
                timeout_ptr,
            )
        };

        if n < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(n as usize)
    }
}

impl Drop for KqueueReactor {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        unsafe {
            syscall1(SYS_CLOSE, self.kqueue_fd as i64);
        }
    }
}

// Syscall helpers
#[cfg(target_os = "macos")]
unsafe fn syscall0(n: i64) -> i64 {
    let ret: i64;
    std::arch::asm!(
        "syscall",
        inlateout("rax") n => ret,
        lateout("rcx") _,
        lateout("r11") _,
    );
    ret
}

#[cfg(target_os = "macos")]
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

unsafe fn kevent_syscall(
    kq: RawFd,
    changelist: *const Kevent,
    nchanges: i32,
    eventlist: *mut Kevent,
    nevents: i32,
    timeout: *const Timespec,
) -> i32 {
    #[cfg(target_os = "macos")]
    {
        let ret: i64;
        std::arch::asm!(
            "syscall",
            inlateout("rax") SYS_KEVENT => ret,
            in("rdi") kq,
            in("rsi") changelist,
            in("rdx") nchanges,
            in("r10") eventlist,
            in("r8") nevents,
            in("r9") timeout,
            lateout("rcx") _,
            lateout("r11") _,
        );
        ret as i32
    }

    #[cfg(not(target_os = "macos"))]
    {
        -1 // Fallback
    }
}

unsafe impl Send for KqueueReactor {}
unsafe impl Sync for KqueueReactor {}
