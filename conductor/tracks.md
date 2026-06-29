# Conductor Tracks

## Track 01 — Workspace foundation

Status: scaffolded

Tasks:

- [x] Create Rust workspace.
- [x] Add core/client/fhir/cli/mcp/export/osd/testkit crates.
- [x] Add Conductor entrypoints.
- [x] Add docs and ADR skeletons.
- [ ] Run `cargo check --workspace --all-targets` in a Rust-enabled environment.

## Track 02 — Access and licensing boundary

Status: scaffolded

Tasks:

- [x] Document bring-your-own-key stance.
- [x] Add export policy model.
- [x] Prohibit real response fixtures by convention.
- [ ] Review Healthpoint API terms for caching, redistribution, attribution, and rate limits.
- [ ] Convert the review into a machine-readable policy file.

## Track 03 — FHIR mapping

Status: scaffolded

Tasks:

- [x] Add synthetic HealthcareService Bundle fixture.
- [x] Map service id/name/provider/locations/type/specialty/telecom.
- [ ] Add Location and Organization fixtures.
- [ ] Add Bundle pagination tests.
- [ ] Decide whether to adopt generated FHIR bindings or keep tolerant JSON mapping.

## Track 04 — CLI

Status: scaffolded

Tasks:

- [x] Add `doctor`.
- [x] Add `search services`.
- [x] Add `get service` and `get organization`.
- [x] Add `export manifest`.
- [ ] Add mock-server integration tests.
- [ ] Add shell completions.

## Track 05 — MCP

Status: scaffolded

Tasks:

- [x] Add rmcp-based stdio server crate.
- [x] Add read-only tool skeletons.
- [ ] Add resource templates.
- [ ] Add MCP prompt templates.
- [ ] Pin rmcp dependency once the selected API surface is validated.

## Track 06 — open_social_data bridge

Status: scaffolded

Tasks:

- [x] Add tabular view adapter crate without hard dependency.
- [ ] Add service/location/code/contact view data dictionaries.
- [ ] Add parquet/Arrow export once local CSV/JSONL is proven.
- [ ] Add `open_social_data` provider integration only when licence review allows.
