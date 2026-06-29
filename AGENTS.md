# Repository Guidelines

## Project structure

- `conductor.json` declares setup and run entrypoints used by Conductor.
- `bin/conductor-setup` links external secrets when available and runs Rust dependency checks.
- `script/server` launches the MCP server.
- `crates/healthpoint-core` contains stable domain types, queries, provenance, and traits.
- `crates/healthpoint-client` contains HTTP/auth logic.
- `crates/healthpoint-fhir` maps FHIR JSON into typed records.
- `crates/healthpoint-cli` defines the terminal UX.
- `crates/healthpoint-mcp` defines the read-only MCP surface.
- `crates/healthpoint-osd-adapter` defines future tabular views.
- `conductor/` stores current context, implementation tracks, and ADRs.

## Build and validation

```bash
bin/conductor-setup
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p healthpoint-cli -- doctor
cargo run -p healthpoint-cli -- search services --text "cervical screening" --limit 5 --format json
```

## Security and data handling

Never commit:

- `.env` or API keys,
- real Healthpoint API payloads,
- local caches,
- logs/traces containing request headers,
- generated CSV/JSONL/Parquet exports.

Use synthetic FHIR fixtures only. Treat all Healthpoint-derived data as licensed and non-redistributable unless `docs/access-and-licensing.md` has been updated with explicit reviewed terms.

## Coding style

Use Rust 2024 edition, thin command handlers, explicit domain types, no unsafe code, and provenance on all retrieved/exported records. Keep FHIR raw JSON available while adding typed projections. Update `conductor/state.json`, `conductor/tracks.md`, and `conductor/checkpoints/` whenever a meaningful implementation pass completes.

## Pull requests

Every PR should include:

- a summary,
- commands run,
- affected Conductor track(s),
- whether any API/licensing assumption changed,
- whether real API data was used locally and how it was kept out of Git.
