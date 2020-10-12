//! Simple wrapper for systemd `sd_journal` API.

#![deny(warnings)]
#![warn(missing_docs)]

mod ffi;

/// Sends a message to the systemd journal.
///
/// # Arguments
///
/// * `msg` - Message string to send.
pub fn send(msg: &str) -> std::io::Result<()> {
    let arg = format!("MESSAGE={}", msg.trim());
    let args = [ffi::IoVec {
        base: arg.as_ptr() as *const _,
        len: arg.len()
    }];
    let ret = unsafe { ffi::sd_journal_sendv(args.as_ptr(), 1) };
    if ret == 0 {
        Ok(())
    } else {
        Err(std::io::Error::from_raw_os_error(ret))
    }
}
