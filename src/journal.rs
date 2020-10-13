//! Contains an abstract for a handle to the systemd journal.

use std::{sync::Mutex, fmt::Write as _};

use crate::ffi;

/// Log priority levels.
#[repr(i32)]
pub enum Priority {
    /// LOG_EMERG from <syslog.h>.
    Emerg = 0,
    /// LOG_ALERT from <syslog.h>.
    Alert = 1,
    /// LOG_Crit from <syslog.h>.
    Crit = 2,
    /// LOG_ERR from <syslog.h>.
    Err = 3,
    /// LOG_WARNING from <syslog.h>.
    Warning = 4,
    /// LOG_NOTICE from <syslog.h>.
    Notice = 5,
    /// LOG_INFO from <syslog.h>.
    Info = 6,
    /// LOG_DEBUG from <syslog.h>.
    Debug = 7
}

/// Handle to the `systemd` journal.
pub struct Journal {
    imp: Mutex<Imp>,
}

impl Journal {
    /// Constructs a new `systemd` journal handle.
    pub fn new() -> Self {
        Journal {
            imp: Mutex::new(Imp::new())
        }
    }

    /// Sends a log entry to the journal.
    pub fn send(&self, pri: Priority, msg: impl std::fmt::Display) -> std::io::Result<()> {
        let mut imp = self.imp.lock().expect("journal lock poisoned");
        imp.set_priority(pri);
        imp.set_message(msg);
        imp.send()
    }
}

struct Imp {
    pri_field: String,
    msg_field: String,
    iovecs: Vec<ffi::IoVec>,
}

impl Imp {
    fn new() -> Self {
        Self {
            pri_field: String::with_capacity(10),
            msg_field: String::with_capacity(64),
            iovecs: Vec::with_capacity(2)
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

    fn prepare(&mut self) -> &[ffi::IoVec] {
        self.iovecs.clear();
        self.iovecs.push(ffi::IoVec::from_str(&self.pri_field));
        self.iovecs.push(ffi::IoVec::from_str(&self.msg_field));
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
