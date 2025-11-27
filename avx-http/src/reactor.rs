//! I/O Reactor - Platform-specific async I/O
//!
//! - Linux: epoll
//! - macOS/BSD: kqueue
//! - Windows: IOCP (I/O Completion Ports)

use crate::error::{Error, Result};
use std::collections::HashMap;
use std::io;
use std::os::raw::c_int;
use std::sync::{Arc, Mutex};
use std::task::Waker;
use std::time::Duration;

/// Interest flags for I/O readiness
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interest {
    /// Readable interest
    pub readable: bool,
    /// Writable interest
    pub writable: bool,
}

impl Interest {
    /// Interest in readable events
    pub const READABLE: Interest = Interest {
        readable: true,
        writable: false,
    };

    /// Interest in writable events
    pub const WRITABLE: Interest = Interest {
        readable: false,
        writable: true,
    };

    /// Interest in both readable and writable events
    pub const READ_WRITE: Interest = Interest {
        readable: true,
        writable: true,
    };
}

/// Event returned by reactor
#[derive(Debug, Clone, Copy)]
pub struct Event {
    /// Token identifying the source
    pub token: usize,
    /// Is readable?
    pub readable: bool,
    /// Is writable?
    pub writable: bool,
}

/// Platform-specific I/O reactor
pub struct Reactor {
    inner: PlatformReactor,
    wakers: Arc<Mutex<HashMap<usize, Waker>>>,
}

impl Reactor {
    /// Create new reactor
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: PlatformReactor::new()?,
            wakers: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Register interest in I/O events
    pub fn register(&mut self, fd: c_int, token: usize, interest: Interest) -> Result<()> {
        self.inner.register(fd, token, interest)
    }

    /// Deregister I/O source
    pub fn deregister(&mut self, fd: c_int) -> Result<()> {
        self.inner.deregister(fd)
    }

    /// Wait for I/O events
    pub fn wait(&mut self, events: &mut Vec<Event>, timeout: Option<Duration>) -> Result<usize> {
        self.inner.wait(events, timeout)
    }

    /// Register a waker for a token
    pub fn register_waker(&self, token: usize, waker: Waker) {
        let mut wakers = self.wakers.lock().unwrap();
        wakers.insert(token, waker);
    }

    /// Wake tasks associated with events
    pub fn wake_events(&self, events: &[Event]) {
        let mut wakers = self.wakers.lock().unwrap();
        for event in events {
            if let Some(waker) = wakers.remove(&event.token) {
                waker.wake();
            }
        }
    }
}

impl Default for Reactor {
    fn default() -> Self {
        Self::new().expect("Failed to create reactor")
    }
}

// ============================================================================
// Linux: epoll
// ============================================================================

#[cfg(target_os = "linux")]
mod platform {
    use super::*;
    use std::os::unix::io::RawFd;

    const EPOLL_CTL_ADD: c_int = 1;
    const EPOLL_CTL_DEL: c_int = 2;
    const EPOLL_CTL_MOD: c_int = 3;

    const EPOLLIN: u32 = 0x001;
    const EPOLLOUT: u32 = 0x004;
    const EPOLLET: u32 = 1 << 31;

    #[repr(C)]
    union EpollData {
        ptr: *mut (),
        fd: c_int,
        u32_: u32,
        u64_: u64,
    }

    #[repr(C)]
    struct EpollEvent {
        events: u32,
        data: EpollData,
    }

    extern "C" {
        fn epoll_create1(flags: c_int) -> c_int;
        fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut EpollEvent) -> c_int;
        fn epoll_wait(epfd: c_int, events: *mut EpollEvent, maxevents: c_int, timeout: c_int) -> c_int;
    }

    pub struct PlatformReactor {
        epoll_fd: RawFd,
    }

    impl PlatformReactor {
        pub fn new() -> Result<Self> {
            let epoll_fd = unsafe { epoll_create1(0) };
            if epoll_fd < 0 {
                return Err(Error::Internal {
                    message: format!("Failed to create epoll: {}", io::Error::last_os_error()),
                });
            }

            Ok(Self { epoll_fd })
        }

        pub fn register(&mut self, fd: c_int, token: usize, interest: Interest) -> Result<()> {
            let mut events = 0u32;
            if interest.readable {
                events |= EPOLLIN;
            }
            if interest.writable {
                events |= EPOLLOUT;
            }
            events |= EPOLLET; // Edge-triggered

            let mut event = EpollEvent {
                events,
                data: EpollData { u64_: token as u64 },
            };

            let result = unsafe { epoll_ctl(self.epoll_fd, EPOLL_CTL_ADD, fd, &mut event) };
            if result < 0 {
                return Err(Error::Internal {
                    message: format!("Failed to register fd: {}", io::Error::last_os_error()),
                });
            }

            Ok(())
        }

        pub fn deregister(&mut self, fd: c_int) -> Result<()> {
            let result = unsafe { epoll_ctl(self.epoll_fd, EPOLL_CTL_DEL, fd, std::ptr::null_mut()) };
            if result < 0 {
                return Err(Error::Internal {
                    message: format!("Failed to deregister fd: {}", io::Error::last_os_error()),
                });
            }

            Ok(())
        }

        pub fn wait(&mut self, events: &mut Vec<Event>, timeout: Option<Duration>) -> Result<usize> {
            let timeout_ms = timeout
                .map(|d| d.as_millis() as c_int)
                .unwrap_or(-1);

            let mut raw_events = vec![
                EpollEvent {
                    events: 0,
                    data: EpollData { u64_: 0 },
                };
                128
            ];

            let n = unsafe {
                epoll_wait(
                    self.epoll_fd,
                    raw_events.as_mut_ptr(),
                    raw_events.len() as c_int,
                    timeout_ms,
                )
            };

            if n < 0 {
                return Err(Error::Internal {
                    message: format!("epoll_wait failed: {}", io::Error::last_os_error()),
                });
            }

            events.clear();
            for i in 0..n as usize {
                let raw_event = &raw_events[i];
                let token = unsafe { raw_event.data.u64_ } as usize;

                events.push(Event {
                    token,
                    readable: (raw_event.events & EPOLLIN) != 0,
                    writable: (raw_event.events & EPOLLOUT) != 0,
                });
            }

            Ok(n as usize)
        }
    }

    impl Drop for PlatformReactor {
        fn drop(&mut self) {
            unsafe {
                libc::close(self.epoll_fd);
            }
        }
    }
}

// ============================================================================
// macOS/BSD: kqueue
// ============================================================================

#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd"))]
mod platform {
    use super::*;
    use std::os::unix::io::RawFd;

    const EVFILT_READ: i16 = -1;
    const EVFILT_WRITE: i16 = -2;
    const EV_ADD: u16 = 0x0001;
    const EV_DELETE: u16 = 0x0002;
    const EV_CLEAR: u16 = 0x0020;

    #[repr(C)]
    struct Kevent {
        ident: usize,
        filter: i16,
        flags: u16,
        fflags: u32,
        data: isize,
        udata: *mut (),
    }

    #[repr(C)]
    struct Timespec {
        tv_sec: isize,
        tv_nsec: isize,
    }

    extern "C" {
        fn kqueue() -> c_int;
        fn kevent(
            kq: c_int,
            changelist: *const Kevent,
            nchanges: c_int,
            eventlist: *mut Kevent,
            nevents: c_int,
            timeout: *const Timespec,
        ) -> c_int;
    }

    pub struct PlatformReactor {
        kqueue_fd: RawFd,
    }

    impl PlatformReactor {
        pub fn new() -> Result<Self> {
            let kqueue_fd = unsafe { kqueue() };
            if kqueue_fd < 0 {
                return Err(Error::Internal {
                    message: format!("Failed to create kqueue: {}", io::Error::last_os_error()),
                });
            }

            Ok(Self { kqueue_fd })
        }

        pub fn register(&mut self, fd: c_int, token: usize, interest: Interest) -> Result<()> {
            let mut changes = Vec::new();

            if interest.readable {
                changes.push(Kevent {
                    ident: fd as usize,
                    filter: EVFILT_READ,
                    flags: EV_ADD | EV_CLEAR,
                    fflags: 0,
                    data: 0,
                    udata: token as *mut (),
                });
            }

            if interest.writable {
                changes.push(Kevent {
                    ident: fd as usize,
                    filter: EVFILT_WRITE,
                    flags: EV_ADD | EV_CLEAR,
                    fflags: 0,
                    data: 0,
                    udata: token as *mut (),
                });
            }

            let result = unsafe {
                kevent(
                    self.kqueue_fd,
                    changes.as_ptr(),
                    changes.len() as c_int,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null(),
                )
            };

            if result < 0 {
                return Err(Error::Internal {
                    message: format!("Failed to register fd: {}", io::Error::last_os_error()),
                });
            }

            Ok(())
        }

        pub fn deregister(&mut self, fd: c_int) -> Result<()> {
            let changes = vec![
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

            let result = unsafe {
                kevent(
                    self.kqueue_fd,
                    changes.as_ptr(),
                    changes.len() as c_int,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null(),
                )
            };

            if result < 0 {
                return Err(Error::Internal {
                    message: format!("Failed to deregister fd: {}", io::Error::last_os_error()),
                });
            }

            Ok(())
        }

        pub fn wait(&mut self, events: &mut Vec<Event>, timeout: Option<Duration>) -> Result<usize> {
            let timeout_spec = timeout.map(|d| Timespec {
                tv_sec: d.as_secs() as isize,
                tv_nsec: d.subsec_nanos() as isize,
            });

            let timeout_ptr = timeout_spec
                .as_ref()
                .map(|t| t as *const Timespec)
                .unwrap_or(std::ptr::null());

            let mut raw_events = vec![
                Kevent {
                    ident: 0,
                    filter: 0,
                    flags: 0,
                    fflags: 0,
                    data: 0,
                    udata: std::ptr::null_mut(),
                };
                128
            ];

            let n = unsafe {
                kevent(
                    self.kqueue_fd,
                    std::ptr::null(),
                    0,
                    raw_events.as_mut_ptr(),
                    raw_events.len() as c_int,
                    timeout_ptr,
                )
            };

            if n < 0 {
                return Err(Error::Internal {
                    message: format!("kevent failed: {}", io::Error::last_os_error()),
                });
            }

            events.clear();
            let mut event_map: HashMap<usize, Event> = HashMap::new();

            for i in 0..n as usize {
                let raw_event = &raw_events[i];
                let token = raw_event.udata as usize;

                let event = event_map.entry(token).or_insert(Event {
                    token,
                    readable: false,
                    writable: false,
                });

                if raw_event.filter == EVFILT_READ {
                    event.readable = true;
                }
                if raw_event.filter == EVFILT_WRITE {
                    event.writable = true;
                }
            }

            events.extend(event_map.into_values());
            Ok(events.len())
        }
    }

    impl Drop for PlatformReactor {
        fn drop(&mut self) {
            unsafe {
                libc::close(self.kqueue_fd);
            }
        }
    }
}

// ============================================================================
// Windows: IOCP (I/O Completion Ports)
// ============================================================================

#[cfg(target_os = "windows")]
mod platform {
    use super::*;
    use std::ptr;
    use std::mem;

    // Windows types
    type HANDLE = *mut std::ffi::c_void;
    type DWORD = u32;
    type ULONG_PTR = usize;
    type LPOVERLAPPED = *mut OVERLAPPED;

    #[repr(C)]
    struct OVERLAPPED {
        internal: ULONG_PTR,
        internal_high: ULONG_PTR,
        offset: DWORD,
        offset_high: DWORD,
        h_event: HANDLE,
    }

    #[repr(C)]
    struct OVERLAPPED_ENTRY {
        lp_completion_key: ULONG_PTR,
        lp_overlapped: LPOVERLAPPED,
        internal: ULONG_PTR,
        dw_number_of_bytes_transferred: DWORD,
    }

    const INVALID_HANDLE_VALUE: HANDLE = (-1isize) as HANDLE;

    // Windows API declarations
    #[link(name = "kernel32")]
    extern "system" {
        fn CreateIoCompletionPort(
            file_handle: HANDLE,
            existing_completion_port: HANDLE,
            completion_key: ULONG_PTR,
            number_of_concurrent_threads: DWORD,
        ) -> HANDLE;

        fn GetQueuedCompletionStatusEx(
            completion_port: HANDLE,
            lp_completion_port_entries: *mut OVERLAPPED_ENTRY,
            ul_count: DWORD,
            ul_num_entries_removed: *mut DWORD,
            dw_milliseconds: DWORD,
            f_alertable: i32,
        ) -> i32;

        fn PostQueuedCompletionStatus(
            completion_port: HANDLE,
            dw_number_of_bytes_transferred: DWORD,
            dw_completion_key: ULONG_PTR,
            lp_overlapped: LPOVERLAPPED,
        ) -> i32;

        fn CloseHandle(h_object: HANDLE) -> i32;
    }

    pub struct PlatformReactor {
        iocp_handle: HANDLE,
        registered_sockets: std::collections::HashMap<c_int, (usize, Interest)>,
    }

    impl PlatformReactor {
        pub fn new() -> Result<Self> {
            unsafe {
                // Create IOCP with unlimited threads
                let iocp = CreateIoCompletionPort(
                    INVALID_HANDLE_VALUE,
                    ptr::null_mut(),
                    0,
                    0,
                );

                if iocp.is_null() {
                    return Err(Error::Internal {
                        message: "Failed to create IOCP".to_string(),
                    });
                }

                Ok(Self {
                    iocp_handle: iocp,
                    registered_sockets: std::collections::HashMap::new(),
                })
            }
        }

        pub fn register(&mut self, fd: c_int, token: usize, interest: Interest) -> Result<()> {
            unsafe {
                // Associate socket with IOCP
                let handle = fd as HANDLE;
                let result = CreateIoCompletionPort(
                    handle,
                    self.iocp_handle,
                    token as ULONG_PTR,
                    0,
                );

                if result.is_null() {
                    return Err(Error::Internal {
                        message: format!("Failed to register fd {} with IOCP", fd),
                    });
                }

                self.registered_sockets.insert(fd, (token, interest));
                Ok(())
            }
        }

        pub fn deregister(&mut self, fd: c_int) -> Result<()> {
            // IOCP doesn't have explicit deregister - just remove from tracking
            self.registered_sockets.remove(&fd);
            Ok(())
        }

        pub fn wait(&mut self, events: &mut Vec<Event>, timeout: Option<Duration>) -> Result<usize> {
            events.clear();

            let timeout_ms = match timeout {
                Some(d) => d.as_millis() as DWORD,
                None => !0u32, // INFINITE
            };

            unsafe {
                const MAX_EVENTS: usize = 64;
                let mut entries: [OVERLAPPED_ENTRY; MAX_EVENTS] = mem::zeroed();
                let mut num_entries: DWORD = 0;

                let result = GetQueuedCompletionStatusEx(
                    self.iocp_handle,
                    entries.as_mut_ptr(),
                    MAX_EVENTS as DWORD,
                    &mut num_entries,
                    timeout_ms,
                    0,
                );

                if result == 0 {
                    // Timeout or error
                    let error = std::io::Error::last_os_error();
                    if error.raw_os_error() == Some(258) { // WAIT_TIMEOUT
                        return Ok(0);
                    }
                    return Err(Error::Internal {
                        message: format!("GetQueuedCompletionStatusEx failed: {}", error),
                    });
                }

                // Process completion entries
                for i in 0..num_entries as usize {
                    let entry = &entries[i];
                    let token = entry.lp_completion_key as usize;

                    // Find interest for this socket
                    let interest = self.registered_sockets
                        .values()
                        .find(|(t, _)| *t == token)
                        .map(|(_, int)| *int)
                        .unwrap_or(Interest::READABLE);

                    let mut event = Event {
                        token,
                        readable: false,
                        writable: false,
                    };

                    // IOCP operations are always ready after completion
                    if interest.contains(Interest::READABLE) {
                        event.readable = true;
                    }
                    if interest.contains(Interest::WRITABLE) {
                        event.writable = true;
                    }

                    events.push(event);
                }

                Ok(num_entries as usize)
            }
        }
    }

    impl Drop for PlatformReactor {
        fn drop(&mut self) {
            unsafe {
                if !self.iocp_handle.is_null() {
                    CloseHandle(self.iocp_handle);
                }
            }
        }
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd", target_os = "openbsd", target_os = "windows")))]
mod platform {
    use super::*;

    pub struct PlatformReactor;

    impl PlatformReactor {
        pub fn new() -> Result<Self> {
            Err(Error::Internal {
                message: "Reactor not supported on this platform".to_string(),
            })
        }

        pub fn register(&mut self, _fd: c_int, _token: usize, _interest: Interest) -> Result<()> {
            Err(Error::Internal {
                message: "Reactor not supported on this platform".to_string(),
            })
        }

        pub fn deregister(&mut self, _fd: c_int) -> Result<()> {
            Err(Error::Internal {
                message: "Reactor not supported on this platform".to_string(),
            })
        }

        pub fn wait(&mut self, _events: &mut Vec<Event>, _timeout: Option<Duration>) -> Result<usize> {
            Err(Error::Internal {
                message: "Reactor not supported on this platform".to_string(),
            })
        }
    }
}

use platform::PlatformReactor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reactor_creation() {
        let result = Reactor::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_interest() {
        let readable = Interest::READABLE;
        assert!(readable.readable);
        assert!(!readable.writable);

        let writable = Interest::WRITABLE;
        assert!(!writable.readable);
        assert!(writable.writable);

        let both = Interest::READ_WRITE;
        assert!(both.readable);
        assert!(both.writable);
    }
}
