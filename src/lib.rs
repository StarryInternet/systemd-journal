//! Simple wrapper for systemd `sd_journal` API.

#![deny(warnings)]
#![warn(missing_docs)]

mod ffi;
mod journal;

pub use journal::{Journal, Priority};

/// Write a formatted log entry to the given `Journal` at log priority `Info`.
///
/// # Examples
///
/// ```no_run
/// # use sd_journal::{Journal, info};
/// # fn main() -> std::io::Result<()> {
/// let jrn = Journal::new();
/// info!(jrn, "2 + 2 = {}", 2 + 2)?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! info {
    ($jrn:expr, $($arg:tt)*) => ($jrn.send($crate::Priority::Info, format_args!($($arg)*)))
}

/// Write a formatted log entry to the given `Journal` at log priority `Warning`.
///
/// # Examples
///
/// ```no_run
/// # use sd_journal::{Journal, warn};
/// # fn main() -> std::io::Result<()> {
/// let jrn = Journal::new();
/// warn!(jrn, "something strange happened")?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    ($jrn:expr, $($arg:tt)*) => ($jrn.send($crate::Priority::Warning, format_args!($($arg)*)))
}

/// Write a formatted log entry to the given `Journal` at log priority `Err`.
///
/// # Examples
///
/// ```no_run
/// # use sd_journal::{Journal, error};
/// # fn main() -> std::io::Result<()> {
/// let jrn = Journal::new();
/// error!(jrn, "something bad happened!")?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! error {
    ($jrn:expr, $($arg:tt)*) => ($jrn.send($crate::Priority::Err, format_args!($($arg)*)))
}
