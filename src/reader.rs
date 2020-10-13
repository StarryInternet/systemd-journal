use crate::ffi;

/// Handle to the systemd journal for reading entries.
pub struct JournalReader {
    jrn_ptr: ffi::JournalPtr
}

impl JournalReader {
    /// Opens the systemd journal for reading.
    pub(crate) fn open(flags: i32) -> std::io::Result<Self> {
        let mut jrn_ptr;
        let ret = unsafe {
            jrn_ptr = ffi::JournalPtr::uninit();
            ffi::sd_journal_open((&mut jrn_ptr) as *mut _, flags as _)
        };
        if ret == 0 {
            Ok(JournalReader { jrn_ptr })
        } else {
            Err(std::io::Error::from_raw_os_error(-ret))
        }
    }
}

impl Drop for JournalReader {
    fn drop(&mut self) {
        unsafe { ffi::sd_journal_close(self.jrn_ptr) }
    }
}
