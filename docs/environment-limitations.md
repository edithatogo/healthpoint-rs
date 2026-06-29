# Environment limitations reached on 2026-06-29

This repository was advanced as far as possible in the current sandbox. The sandbox can read/write files, run Python and shell checks, create Git commits, and package artifacts, but it does not have a Rust toolchain installed.

## Completed in the sandbox

- Rust workspace and crate layout are present.
- Conductor setup, tracks, decisions, checkpoints, and run scripts are present.
- JSON and TOML metadata parse successfully.
- `git diff --check` passes.
- A crude Rust delimiter scan passes.
- A simple high-signal secret-pattern scan passes.
- `bin/conductor-setup` supports metadata-only mode with `CONDUCTOR_ALLOW_NO_CARGO=1`.
- Source ZIP and Git bundle artifacts can be generated from committed state.

## Not possible in the sandbox

- `cargo fmt --all --check`
- `cargo check --workspace --all-targets`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo deny check`
- `Cargo.lock` generation
- Compile-time validation of the bleeding-edge `rmcp` API surface
- Live Healthpoint API validation with a licensed API key

An attempt to install Rust through system package tooling timed out because repository/network access was unavailable. Treat this repository as a strong implementation spike until the Rust validation lane passes.

## Next validation lane

Run these commands from the repository root in a Rust-enabled environment:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check
```

Then run metadata/Conductor checks:

```bash
bin/conductor-setup
cargo run -p healthpoint-cli -- doctor
cargo run -p healthpoint-cli -- fixture services --format json
cargo run -p healthpoint-cli -- schema service-record
```

## Live validation lane

Only after offline Rust validation passes, run live tests with a licensed Healthpoint key. Do not commit live payloads.

```bash
export HEALTHPOINT_BASE_URL="https://uat.healthpointapi.com/baseR4/"
export HEALTHPOINT_API_KEY="..."
export HEALTHPOINT_AUTH_SCHEME="x-api-key"

cargo run -p healthpoint-cli -- doctor
cargo run -p healthpoint-cli -- inspect search-url --snomed 171149006 --limit 5
cargo run -p healthpoint-cli -- search services --snomed 171149006 --limit 5 --format json
```
