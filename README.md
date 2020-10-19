# `systemd-journal`

A simple, zero-dependency crate for working with Systemd's
`sd_journal` API's.

## Code examples

`systemd_journal` provides an interface to the Systemd journal through
the `Journal` type.

```rust
let jrn = systemd_journal::Journal::new();

systemd_journal::info!(jrn, "Hello, world!")?;
if let Err(err) = something::dangerous() {
  systemd_journal::error!(jrn, "Oops: {}", err)?;
}

let mut reader = jrn.read()?;
if let Some(mut entry) = reader.next()? {
  if let Some(message) = entry.get("MESSAGE")? {
    println!("First journal entry: {:?}", String::from_utf8_lossy(message));
  }
}
```

## Integration with `log`

`systemd_journal` can be used to set up the global logger for the
popular [log][docs_log] crate.

[docs_log]: https://docs.rs/crate/log

```rust
fn main() {
     systemd_journal::init_logger();
     log::info!("Hello, world!");
}
```

Note: this requires enabling the `"log"` feature in `Cargo.toml`:

```toml
[dependencies.systemd-journal]
version = "0.1"
features = ["log"]
```

# License

This project is licensed under the MIT license (see the `LICENSE` file).
