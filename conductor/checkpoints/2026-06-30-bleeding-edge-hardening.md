# Checkpoint: bleeding-edge hardening

Date: 2026-06-30

## Summary

The repository dependency, CI, release, and code-quality posture was upgraded after the initial local setup validation.

## Changes

- Raised workspace MSRV to Rust 1.88 while keeping `rust-toolchain.toml` on stable.
- Upgraded `reqwest` to 0.13 with explicit Rustls/WebPKI features.
- Refreshed `Cargo.lock` to latest Rust 1.88-compatible dependency resolution.
- Updated GitHub Actions to Dependabot-proposed current majors:
  - `actions/checkout@v7`
  - `actions/upload-artifact@v7`
  - `gitleaks/gitleaks-action@v3`
- Added cross-platform Rust CI on Linux, macOS, and Windows.
- Added `.gitattributes` LF policy so rustfmt checks are stable on Windows.
- Added an explicit Rust 1.88 MSRV check.
- Switched CI and docs to locked Cargo commands.
- Increased Dependabot cadence to daily for Cargo and GitHub Actions.
- Added Renovate semantic commits, digest automerge, timezone, and weekday schedule.

## Validation target

```bash
bin/conductor-setup
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo deny check
```
