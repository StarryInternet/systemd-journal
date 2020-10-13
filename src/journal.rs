//! Contains an abstract for a handle to the systemd journal.

use std::{fmt::Write as _, sync::Mutex};

use crate::{ffi, reader::JournalReader};

/// Log priority levels.
#[repr(i32)]
pub enum Priority {
    /// `LOG_EMERG` from <syslog.h>.
    Emerg = 0,
    /// `LOG_ALERT` from <syslog.h>.
    Alert = 1,
    /// `LOG_CRIT` from <syslog.h>.
    Crit = 2,
    /// `LOG_ERR` from <syslog.h>.
    Err = 3,
    /// `LOG_WARNING` from <syslog.h>.
    Warning = 4,
    /// `LOG_NOTICE` from <syslog.h>.
    Notice = 5,
    /// `LOG_INFO` from <syslog.h>.
    Info = 6,
    /// `LOG_DEBUG` from <syslog.h>.
    Debug = 7
}

/// Handle to the `systemd` journal.
pub struct Journal {
    imp: Mutex<Imp>
}

impl Journal {
    /// Constructs a new `systemd` journal handle.
    pub fn new() -> Self {
        Self {
            imp: Mutex::new(Imp::new())
        }
    }

    /// Constructs a new `systemd` journal handle, that is configured to send journal logs on behalf
    /// of another process.
    ///
    /// See `OBJECT_PID` in `systemd.journal-fields` man page for more information on this setting.
    ///
    /// # Arguments
    ///
    /// * `pid` - OS-assigned process identifier for the target process.
    pub fn with_object_pid(pid: u32) -> Self {
        let mut imp = Imp::new();
        imp.set_process_id(pid);
        Self {
            imp: Mutex::new(imp)
        }
    }

    /// Sends a log entry to the journal.
    ///
    /// # Arguments
    ///
    /// * `pri` - Log priority level.
    /// * `msg` - Log message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use sd_journal::{Priority, Journal};
    /// # fn main() -> std::io::Result<()> {
    /// let jrn = Journal::new();
    /// jrn.send(Priority::Info, "Hello, world!")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn send(&self, pri: Priority, msg: impl std::fmt::Display) -> std::io::Result<()> {
        let mut imp = self.imp.lock().expect("journal lock poisoned");
        imp.set_priority(pri);
        imp.set_message(msg);
        imp.send()
    }

    /// Opens the journal for reading, returning a handle for reading journal entries.
    pub fn read(&self) -> std::io::Result<JournalReader> {
        let flags = 0;
        JournalReader::open(flags)
    }
}

struct Imp {
    pri_field: String,
    msg_field: String,
    pid_field: String,
    iovecs: Vec<ffi::IoVec>
}

impl Imp {
    fn new() -> Self {
        Self {
            pri_field: String::with_capacity(10),
            msg_field: String::with_capacity(64),
            pid_field: String::new(),
            iovecs: Vec::with_capacity(3)
        }
    }

    fn set_priority(&mut self, pri: Priority) {
        self.pri_field.clear();
        write!(self.pri_field, "PRIORITY={}", pri as i32).unwrap();
    }

    fn set_message(&mut self, msg: impl std::fmt::Display) {
        self.msg_field.clear();
        write!(self.msg_field, "MESSAGE={}", msg).unwrap();
    }

    fn set_process_id(&mut self, pid: u32) {
        self.pid_field.clear();
        write!(self.pid_field, "OBJECT_PID={}", pid).unwrap();
    }

    fn prepare(&mut self) -> &[ffi::IoVec] {
        self.iovecs.clear();
        self.iovecs.push(ffi::IoVec::from_str(&self.pri_field));
        self.iovecs.push(ffi::IoVec::from_str(&self.msg_field));
        if !self.pid_field.is_empty() {
            self.iovecs.push(ffi::IoVec::from_str(&self.pid_field));
        }
        &self.iovecs
    }

    fn send(&mut self) -> std::io::Result<()> {
        let ret = unsafe {
            let iovecs = self.prepare();
            ffi::sd_journal_sendv(iovecs.as_ptr(), iovecs.len() as _)
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(std::io::Error::from_raw_os_error(ret))
        }
    }
}

#[cfg(feature = "log")]
impl log::Log for Journal {
    fn enabled(&self, meta: &log::Metadata) -> bool {
        meta.level() != log::Level::Trace
    }

    fn log(&self, rec: &log::Record) {
        let pri = match rec.level() {
            log::Level::Trace => unreachable!(),
            log::Level::Debug => Priority::Debug,
            log::Level::Info => Priority::Info,
            log::Level::Warn => Priority::Warning,
            log::Level::Error => Priority::Err
        };
        self.send(pri, rec.args()).expect("journal send failed");
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod test {
    #[cfg(feature = "log")]
    fn _set_logger_typechecks() {
        use super::Journal;
        let jrn = Journal::new();
        let jrn: &'static Journal = unsafe { &*(&jrn as *const _) };
        log::set_logger(jrn).unwrap();
    }
}
