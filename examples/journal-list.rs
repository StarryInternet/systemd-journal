use systemd_journal::Journal;

fn main() -> std::io::Result<()> {
    let jrn = Journal::new();
    let mut rdr = jrn.read()?;
    let mut cnt = 0;
    while let Some(mut ent) = rdr.next()? {
        let msg = ent.get("MESSAGE")?.unwrap_or(b"(no message)");
        let msg = String::from_utf8_lossy(msg);
        println!("{:3}. {}", cnt + 1, msg);

        cnt += 1;
        if cnt >= 100 {
            break;
        }
    }
    Ok(())
}
