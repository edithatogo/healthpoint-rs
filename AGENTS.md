# Repository Guidelines

## Project structure

This repository is a Rust workspace for Healthpoint API tooling.

- `conductor.json` declares setup/run/context entrypoints used by Conductor.
- `bin/conductor-setup` performs metadata preflight and full Cargo setup when Cargo is available.
- `script/server` starts the read-only MCP server from source.
- `conductor/` stores state, decisions, checkpoints, and implementation tracks.
- `crates/healthpoint-core/` contains domain records, query model, validation, redaction, provenance, and resource URIs.
- `crates/healthpoint-fhir/` contains tolerant FHIR JSON mappers.
- `crates/healthpoint-client/` contains the async HTTP client.
- `crates/healthpoint-cli/` defines the `healthpoint` command surface.
- `crates/healthpoint-mcp/` defines the read-only MCP server.
- `crates/healthpoint-export/` contains export helpers and manifests.
- `crates/healthpoint-osd-adapter/` contains future open_social_data tabular views.
- `crates/healthpoint-testkit/` contains synthetic fixtures only.

## Build, test, and development commands

Run from the repository root:

```bash
bin/conductor-setup
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p healthpoint-cli -- fixture services --format human
cargo run -p healthpoint-cli -- doctor
cargo run -p healthpoint-mcp
```

If Cargo is unavailable and you only need metadata validation:

```bash
CONDUCTOR_ALLOW_NO_CARGO=1 bin/conductor-setup
```

## Coding style

Use safe Rust only. Keep command handlers thin and delegate domain logic to crates. Preserve raw FHIR JSON on typed records. Avoid adding heavyweight dependencies to `healthpoint-core`.

## Testing guidelines

- Use synthetic fixtures in CI.
- Do not commit real Healthpoint API responses.
- Gate live tests behind explicit environment variables and document metadata-only findings.
- Before changing FHIR mapping, update fixtures and schema/adapter docs.

## Security and configuration

Do not commit `.env`, API keys, local caches, generated exports, traces, logs, databases, Parquet files, JSONL exports, or real API payloads. Secret values must be redacted from diagnostics and errors.

## Conductor rules

- Update `conductor/state.json` after meaningful implementation passes.
- Add a checkpoint under `conductor/checkpoints/` before handoff.
- Update track files when scope changes.
- Add ADRs for architectural decisions that affect integrations, schema stability, licensing posture, or MCP surface.
