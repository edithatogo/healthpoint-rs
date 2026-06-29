# Development environment

## Native Rust

Preferred:

```bash
rustup toolchain install stable --component rustfmt,clippy
bin/conductor-setup
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo deny check
```

## Dev container

A devcontainer and Dockerfile are included for environments where Rust is not installed locally.

```bash
docker build -t healthpoint-rs-dev .
docker run --rm -it -v "$PWD:/workspace" -w /workspace healthpoint-rs-dev bash
```

Inside the container:

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
```

## Metadata-only setup

For orchestration/sandbox environments without Rust:

```bash
CONDUCTOR_ALLOW_NO_CARGO=1 bin/conductor-setup
```

This validates JSON/TOML metadata and exits before Cargo commands.
