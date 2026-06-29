# Development environment

## Native Rust

Preferred:

```bash
rustup toolchain install stable --component rustfmt,clippy
bin/conductor-setup
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Dev container

A devcontainer and Dockerfile are included for environments where Rust is not installed locally.

```bash
docker build -t healthpoint-rs-dev .
docker run --rm -it -v "$PWD:/workspace" -w /workspace healthpoint-rs-dev bash
```

Inside the container:

```bash
cargo check --workspace --all-targets
cargo test --workspace
```

## Metadata-only setup

For orchestration/sandbox environments without Rust:

```bash
CONDUCTOR_ALLOW_NO_CARGO=1 bin/conductor-setup
```

This validates JSON/TOML metadata and exits before Cargo commands.
