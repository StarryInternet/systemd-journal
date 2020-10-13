use std::{ffi::c_void, os::raw::c_int};

/// Identical to `struct iovec` from C's <uio.h>.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IoVec {
    pub base: *const c_void,
    pub len: usize
}

unsafe impl Send for IoVec {}

impl IoVec {
    pub fn from_str(s: &str) -> Self {
        Self {
            base: s.as_ptr() as *const _,
            len: s.len()
        }
    }
}

extern "C" {
    pub fn sd_journal_sendv(iov: *const IoVec, n: c_int) -> c_int;
}
