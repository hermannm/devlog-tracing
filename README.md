# devlog-tracing

Log subscriber for Rust's [`tracing`](https://github.com/tokio-rs/tracing) library, with a
human-readable output format designed for development builds. Uses the same format as the
[`devlog`](https://github.com/hermannm/devlog) library for Go.

Run `cargo add devlog-tracing` to add it to your project!

**Contents:** [crates.io/crates/devlog-tracing](https://crates.io/crates/devlog-tracing)

**Contents:**

- [Usage](#usage)
- [Maintainer's guide](#maintainers-guide)

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

## Maintainer's guide

Publishing a new release:

- Run tests:
  ```
  cargo test
  ```
- Add an entry to `CHANGELOG.md` (with the current date)
    - Remember to update the link section, and bump the version for the `[Unreleased]` link
- Create commit and tag for the release (update `TAG` variable in below command):
  ```
  TAG=vX.Y.Z && git commit -m "Release ${TAG}" && git tag -a "${TAG}" -m "Release ${TAG}" && git log --oneline -2
  ```
- Push the commit and tag:
  ```
  git push && git push --tags
  ```
    - Our release workflow will then create a GitHub release with the pushed tag's changelog entry
