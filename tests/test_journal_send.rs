extern crate sd_journal;

use sd_journal::{error, info, Journal};

static INFO_STRING: &str = "Hello from sd_journal integration test";
static ERROR_STRING: &str = "Watch out, its sd_journal integration test!";

#[test]
fn test_journal_send() {
    let jrn = Journal::new();
    info!(jrn, "{}", INFO_STRING).unwrap();

    let mut cmd = std::process::Command::new("journalctl");
    cmd.args(&["-r", "-n1", "-p6", "--no-pager"]);
    let stdout = cmd.output().unwrap().stdout;
    let stdout = String::from_utf8_lossy(&stdout);
    assert!(stdout.contains(INFO_STRING), "{:?}", stdout);
}

#[test]
fn test_journal_error() {
    let jrn = Journal::new();
    error!(jrn, "{}", ERROR_STRING).unwrap();

    let mut cmd = std::process::Command::new("journalctl");
    cmd.args(&["-r", "-n1", "-p3", "--no-pager"]);
    let stdout = cmd.output().unwrap().stdout;
    let stdout = String::from_utf8_lossy(&stdout);
    assert!(stdout.contains(ERROR_STRING), "{:?}", stdout);
}
