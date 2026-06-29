# Conductor Tracks

## Status summary

| Track | Status | Current position |
| --- | --- | --- |
| 01. Workspace foundation | Partial | Workspace, CI, Conductor files, docs, and crates exist; Rust validation still needs a Rust-enabled environment. |
| 02. Access and licensing boundary | Partial | Bring-your-own-key and local-only defaults are encoded; terms review and machine-readable policy remain open. |
| 03. FHIR mapping | Partial | Synthetic HealthcareService, Location, Organization, and pagination mapping are implemented; live profile validation remains open. |
| 04. CLI | Partial | doctor/search/get/export manifest are scaffolded with JSON/JSONL/CSV/human service output; completions and mock-server tests remain open. |
| 05. MCP | Partial | RMCP stdio server exposes read-only tools; resources/prompts remain planned. |
| 06. open_social_data bridge | Partial | Tabular view adapter exists without hard dependency; licensing and Polars/Arrow integration remain open. |
| 07. Live validation | Planned | Requires Dylan's Healthpoint API key and licensed documentation; do not commit payloads. |

## Track 01 — Workspace foundation

Status: partial

Tasks:

- [x] Create Rust workspace.
- [x] Add core/client/fhir/cli/mcp/export/osd/testkit crates.
- [x] Add Conductor entrypoints.
- [x] Add docs and ADR skeletons.
- [x] Add CI, cargo-deny, rustfmt, clippy, and justfile scaffolding.
- [ ] Run `cargo check --workspace --all-targets` in a Rust-enabled environment.
- [ ] Run `cargo test --workspace` in a Rust-enabled environment.

## Track 02 — Access and licensing boundary

Status: partial

Tasks:

- [x] Document bring-your-own-key stance.
- [x] Add export policy model.
- [x] Prohibit real response fixtures by convention.
- [x] Add same-origin guard for absolute pagination cursors.
- [ ] Review Healthpoint API terms for caching, redistribution, attribution, and rate limits.
- [x] Add conservative machine-readable policy stub.
- [ ] Convert the terms review into an updated machine-readable policy file.

## Track 03 — FHIR mapping

Status: partial

Tasks:

- [x] Add synthetic HealthcareService Bundle fixture.
- [x] Add Location and Organization fixtures.
- [x] Map service id/name/provider/locations/type/specialty/telecom.
- [x] Map coverage areas, program, communication, referral methods, and appointment requirement.
- [x] Map Location address/position/managing organization.
- [x] Add Bundle pagination tests.
- [ ] Decide whether to adopt generated FHIR bindings or keep tolerant JSON mapping after live-profile validation.

## Track 04 — CLI

Status: partial

Tasks:

- [x] Add `doctor`.
- [x] Add `search services`.
- [x] Add SNOMED convenience search.
- [x] Add nearby-search flags.
- [x] Add `get service`, `get organization`, and `get location`.
- [x] Add `export manifest`.
- [x] Add JSONL and CSV service output.
- [ ] Add mock-server integration tests.
- [ ] Add shell completions.

## Track 05 — MCP

Status: partial

Tasks:

- [x] Add rmcp-based stdio server crate.
- [x] Add read-only tool skeletons.
- [x] Add diagnostic/status, SNOMED, nearby, service, location, and organization tools.
- [ ] Add resource templates.
- [ ] Add MCP prompt templates.
- [ ] Pin rmcp dependency once the selected API surface is validated.

## Track 06 — open_social_data bridge

Status: partial

Tasks:

- [x] Add tabular view adapter crate without hard dependency.
- [x] Add service/location/code/contact row views.
- [ ] Add service/location/code/contact view data dictionaries.
- [ ] Add parquet/Arrow export once local CSV/JSONL is proven.
- [ ] Add `open_social_data` provider integration only when licence review allows.

## Track 07 — Live validation

Status: planned

Tasks:

- [ ] Confirm base URL and auth scheme.
- [ ] Confirm search parameter names.
- [ ] Confirm pagination shape.
- [ ] Confirm nearby-search encoding.
- [ ] Confirm direct reads for HealthcareService, Location, and Organization.
- [ ] Record only metadata and endpoint-shape notes; never commit real Healthpoint payloads.
