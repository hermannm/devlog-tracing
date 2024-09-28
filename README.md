# devlog-tracing

Log subscriber for Rust's [`tracing`](https://github.com/tokio-rs/tracing) library, with a
human-readable output format designed for development builds. Uses the same format as the
[`devlog`](https://github.com/hermannm/devlog) library for Go.

## Usage

`devlog-tracing` is pretty much a drop-in replacement for
[`tracing-subscriber`](https://github.com/tokio-rs/tracing/tree/master/tracing-subscriber#readme),
so the initialization works the same - just replace `tracing_subscriber::fmt()` with
`devlog_tracing::subscriber()`:

```rust
devlog_tracing::subscriber().init();
```

`tracing` logs will now be formatted by the `devlog-tracing` subscriber:

```rust
// In module `app::server`
warn!("No value found for 'PORT' in env, defaulting to 8000");
info!(port = 8000, environment = "DEV", "Server started");

// In module `app::db`
error!(cause = "UNKNOWN_TABLE", "Database query failed");
```

...giving the following output (using a gruvbox terminal color scheme):

![Screenshot of log messages in a terminal](https://github.com/hermannm/devlog-tracing/blob/372bbd5d08bac0c900d6124d36f4af2efc398dfe/devlog-tracing-example-output.png?raw=true)
