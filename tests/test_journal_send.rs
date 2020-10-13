extern crate sd_journal;

static STRING_TO_LOG: &str = "Hello from sd_journal integration test";

#[test]
fn test_journal_send() {
    let jrn = sd_journal::Journal::new();
    sd_journal::info!(jrn, "{}", STRING_TO_LOG).unwrap();

    let mut cmd = std::process::Command::new("journalctl");
    cmd.args(&["-r", "-n2", "--no-pager"]);
    let stdout = cmd.output().unwrap().stdout;
    let stdout = String::from_utf8_lossy(&stdout);
    assert!(stdout.contains(STRING_TO_LOG), "{:?}", stdout);
}
