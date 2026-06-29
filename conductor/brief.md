# Conductor brief: healthpoint-rs

## Mission

Build a Rust-first Healthpoint API SDK, CLI, and read-only MCP server that can later integrate with `open_social_data` without assuming Healthpoint-derived data is open or redistributable.

## Current architecture

```text
healthpoint-core
  Domain records, query model, provenance, access policy, validation, redaction, resource URIs.

healthpoint-fhir
  Tolerant FHIR R4 JSON mapping for HealthcareService, Location, and Organization.

healthpoint-client
  Async reqwest client with configurable base URL, auth scheme, search URL building, response metadata, and same-origin cursor handling.

healthpoint-cli
  doctor/search/get/get-uri/inspect/export/fixture/schema/policy command surface.

healthpoint-mcp
  Read-only RMCP stdio server over the same client/core.

healthpoint-export
  JSON/JSONL/CSV service exports and export manifests.

healthpoint-osd-adapter
  Future open_social_data tabular view adapter without hard dependency.

healthpoint-testkit
  Synthetic fixtures and offline FixtureDirectoryProvider.
```

## Non-negotiables

- No real Healthpoint payloads in Git.
- No committed API keys or local caches.
- No scraping fallback.
- No public proxy mode.
- No open-data claims until terms/permission allow it.
- MCP remains read-only.
- Exports default to local-only and carry manifests.

## Current environment state

The current sandbox has no Cargo/Rust. Metadata validation passes, but compile/test/clippy must run elsewhere.

## Next handoff command sequence

```bash
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo run -p healthpoint-cli -- fixture services --format human
cargo run -p healthpoint-cli -- inspect search-url --text "cervical screening" --snomed 171149006
```

## Live validation rule

Use Dylan's Healthpoint key locally. Record endpoint-shape metadata only; do not commit API responses.
