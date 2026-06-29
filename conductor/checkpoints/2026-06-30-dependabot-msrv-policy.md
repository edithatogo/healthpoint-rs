# Dependabot MSRV policy cleanup

Date: 2026-06-30

## Summary

Dependabot opened PR #4 to bump `dtolnay/rust-toolchain` from the intentional MSRV pin `1.88.0` to `1.100.0`. The PR failed the MSRV job, which is expected because the workflow pin represents the repository's minimum supported Rust version, not an ordinary action dependency.

## Action taken

- Added a Dependabot ignore rule for `dtolnay/rust-toolchain` under GitHub Actions updates.
- Left the MSRV workflow pin at `1.88.0`.
- Closed the obsolete Dependabot PR after recording the policy decision.

## Follow-up

Raise the MSRV deliberately in a normal implementation pass when the workspace requires a newer compiler and update `Cargo.toml`, `clippy.toml`, CI, docs, and Conductor state together.
