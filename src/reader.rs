use std::ffi::{CStr, CString};

use crate::ffi;

/// Handle to the `systemd` journal for reading entries.
pub struct JournalReader {
    jrn_ptr: ffi::JournalPtr
}

/// Handle to a single entry in the `systemd` journal.
pub struct JournalEntry<'jrn> {
    jrn_ptr: ffi::JournalPtr,
    _marker: std::marker::PhantomData<&'jrn JournalReader>
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

    /// Returns the next entry in the journal, or `None` if the last entry has been reached.
    ///
    /// Unfortunately this function must be used explicitly instead of an `Iterator` implementation
    /// being available. This is mainly because the `systemd` API has specific lifetime requirements
    /// where previous entries become invalidated when you advance to a new entry. On the other
    /// hand, this allows buffers to potentially be reused internally.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use systemd_journal::{Journal, JournalReader};
    /// # fn main() -> std::io::Result<()> {
    /// let jrn = Journal::new();
    /// let mut reader = jrn.read()?;
    /// let mut entry_count = 0;
    /// while reader.next()?.is_some() {
    ///     entry_count += 1;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn next(&mut self) -> std::io::Result<Option<JournalEntry>> {
        let ret = unsafe { ffi::sd_journal_next(self.jrn_ptr) };
        if ret == 0 {
            Ok(None)
        } else if ret > 0 {
            Ok(Some(JournalEntry {
                jrn_ptr: self.jrn_ptr,
                _marker: std::marker::PhantomData
            }))
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

impl<'jrn> JournalEntry<'jrn> {
    fn get_data_raw(&mut self, field: &CStr) -> std::io::Result<Option<&[u8]>> {
        let mut data = std::ptr::null();
        let mut length = 0;
        let ret = unsafe {
            ffi::sd_journal_get_data(self.jrn_ptr, field.as_ptr(), &mut data, &mut length)
        };
        if ret == 0 {
            let data = unsafe { std::slice::from_raw_parts(data, length) };
            let skip_len = field.to_bytes().len() + 1;
            Ok(Some(&data[skip_len..]))
        } else if ret == -2 {
            // -ENOENT
            Ok(None)
        } else {
            Err(std::io::Error::from_raw_os_error(-ret))
        }
    }

    /// Returns the data for the specified field for this entry, if any.
    ///
    /// # Arguments
    ///
    /// * `field` - The field name, e.g. `"MESSAGE"`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use systemd_journal::{Journal, JournalReader, JournalEntry};
    /// # fn main() -> std::io::Result<()> {
    /// let jrn = Journal::new();
    /// let mut reader = jrn.read()?;
    /// while let Some(mut ent) = reader.next()? {
    ///     if let Some(msg) = ent.get("MESSAGE")? {
    ///         println!("- {}", String::from_utf8_lossy(msg));
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&mut self, field: &str) -> std::io::Result<Option<&[u8]>> {
        let field = CString::new(field.as_bytes()).map_err(|_| std::io::ErrorKind::InvalidInput)?;
        self.get_data_raw(&field)
    }
}
