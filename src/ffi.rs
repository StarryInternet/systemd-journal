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

/// Used to represent `sd_journal*` pointers.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct JournalPtr {
    _ptr: *const c_void
}

impl JournalPtr {
    pub unsafe fn uninit() -> Self {
        Self {
            _ptr: std::mem::MaybeUninit::uninit().as_ptr()
        }
    }
}

extern "C" {
    pub fn sd_journal_sendv(iov: *const IoVec, n: c_int) -> c_int;

    pub fn sd_journal_open(out_jrn: *mut JournalPtr, flags: c_int) -> c_int;
    pub fn sd_journal_close(jrn: JournalPtr);
}
