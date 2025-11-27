//! # IOCP - Windows native event loop
//!
//! Implementação usando WinAPI direto:
//! - CreateIoCompletionPort
//! - GetQueuedCompletionStatus
//! - PostQueuedCompletionStatus

use std::io;
use std::ptr;
use std::time::Duration;
use crate::reactor::Interest;

#[allow(non_camel_case_types)]
type Handle = *mut std::ffi::c_void;
#[allow(non_camel_case_types)]
type Dword = u32;
#[allow(non_camel_case_types)]
type UlongPtr = usize;
#[allow(non_camel_case_types)]
type Overlapped = std::ffi::c_void;

const INVALID_HANDLE_VALUE: Handle = -1isize as Handle;
const INFINITE: Dword = 0xFFFFFFFF;

pub struct IocpReactor {
    iocp: Handle,
}

impl IocpReactor {
    pub fn new() -> io::Result<Self> {
        let iocp = unsafe {
            CreateIoCompletionPort(
                INVALID_HANDLE_VALUE,
                ptr::null_mut(),
                0,
                0,
            )
        };

        if iocp.is_null() {
            return Err(io::Error::last_os_error());
        }

        Ok(Self { iocp })
    }

    pub fn register(&self, fd: i32, token: usize, _interest: Interest) -> io::Result<()> {
        // No Windows, fd é um Handle
        let handle = fd as Handle;

        let ret = unsafe {
            CreateIoCompletionPort(
                handle,
                self.iocp,
                token as UlongPtr,
                0,
            )
        };

        if ret.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn deregister(&self, _fd: i32) -> io::Result<()> {
        // IOCP não tem deregister explícito
        // Handles são automaticamente removidos quando fechados
        Ok(())
    }

    pub fn wake(&self) {
        // Posta completion packet com token especial
        unsafe {
            PostQueuedCompletionStatus(
                self.iocp,
                0,
                usize::MAX as UlongPtr,
                ptr::null_mut(),
            );
        }
    }

    pub fn wait(&self, timeout: Option<Duration>) -> io::Result<usize> {
        let timeout_ms = timeout
            .map(|d| d.as_millis() as Dword)
            .unwrap_or(INFINITE);

        let mut bytes_transferred: Dword = 0;
        let mut completion_key: UlongPtr = 0;
        let mut overlapped: *mut Overlapped = ptr::null_mut();

        let ret = unsafe {
            GetQueuedCompletionStatus(
                self.iocp,
                &mut bytes_transferred,
                &mut completion_key,
                &mut overlapped,
                timeout_ms,
            )
        };

        if ret == 0 {
            let err = io::Error::last_os_error();
            if err.raw_os_error() == Some(258) { // WAIT_TIMEOUT
                return Ok(0);
            }
            return Err(err);
        }

        Ok(1)
    }
}

impl Drop for IocpReactor {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.iocp);
        }
    }
}

unsafe impl Send for IocpReactor {}
unsafe impl Sync for IocpReactor {}

// WinAPI declarations
#[link(name = "kernel32")]
extern "system" {
    fn CreateIoCompletionPort(
        file_handle: Handle,
        existing_completion_port: Handle,
        completion_key: UlongPtr,
        number_of_concurrent_threads: Dword,
    ) -> Handle;

    fn GetQueuedCompletionStatus(
        completion_port: Handle,
        lpnumber_of_bytes_transferred: *mut Dword,
        lpcompletion_key: *mut UlongPtr,
        lpoverlapped: *mut *mut Overlapped,
        dwmilliseconds: Dword,
    ) -> i32;

    fn PostQueuedCompletionStatus(
        completion_port: Handle,
        dwnumber_of_bytes_transferred: Dword,
        dwcompletion_key: UlongPtr,
        lpoverlapped: *mut Overlapped,
    ) -> i32;

    fn CloseHandle(hobject: Handle) -> i32;
}
