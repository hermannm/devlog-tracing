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

After this, log events produced with `tracing` will be formatted by the `devlog-tracing` subscriber.
