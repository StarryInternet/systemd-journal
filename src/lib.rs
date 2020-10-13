//! Simple wrapper for systemd `sd_journal` API.

#![deny(warnings)]
#![warn(missing_docs)]

mod ffi;
mod journal;
mod reader;

pub use journal::{Journal, Priority};
pub use reader::{JournalEntry, JournalReader};

/// Write a formatted log entry to the given `Journal` at log priority `Info`.
///
/// # Examples
///
/// ```no_run
/// # use systemd_journal::{Journal, info};
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
/// # use systemd_journal::{Journal, warn};
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
/// # use systemd_journal::{Journal, error};
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

/// Initializes the global logger with a the given `systemd` journal logger. This function should
/// only be called once.
///
/// # Arguments
///
/// * `jrn` - The journal handle to use for logging.
#[cfg(feature = "log")]
pub fn init_logger_with(jrn: Journal) {
    let jrn = Box::leak(Box::new(jrn));
    log::set_logger(jrn).expect("failed to set logger")
}

/// Initializes the global logger with a new `systemd` journal logger. This function should only be
/// called once.
#[cfg(feature = "log")]
pub fn init_logger() {
    init_logger_with(Journal::new());
}
