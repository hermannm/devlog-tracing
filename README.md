# devlog-tracing

Log subscriber for the [`tracing`](https://github.com/tokio-rs/tracing) Rust logging library, using
the same log format as the [`devlog`](https://github.com/hermannm/devlog) library for Go.

## Usage

`devlog-tracing` is pretty much a drop-in replacement for
[`tracing-subscriber`](https://github.com/tokio-rs/tracing/tree/master/tracing-subscriber#readme),
so the initialization works the same - just replace `tracing_subscriber` with `devlog_tracing`:

```rust
devlog_tracing::fmt().init();
```

After this, log events produced with `tracing` will be formatted by the `devlog-tracing` subscriber.
