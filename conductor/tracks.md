# Conductor Tracks

## Status summary

| Track | Status | Current position |
| --- | --- | --- |
| 01. Workspace foundation | Complete | Workspace, CI, Conductor files, docs, crates, static preflight, mock server, Cargo.lock, setup, fmt, locked check/test/clippy, and cargo-deny validate locally. |
| 02. Access and licensing boundary | Complete | Bring-your-own-key/local-only defaults, redaction, path validation, manifest warnings, portal terms review, VUW Outlook licence evidence, and machine-readable policy are encoded. |
| 03. FHIR mapping | Partial | Synthetic HealthcareService, Location, Organization, pagination, richer availability/eligibility/identifier mappings are implemented; live profile validation remains open. |
| 04. CLI | Complete | doctor/search/get/get-uri/inspect/export/fixture/schema/policy/completions commands and mock-backed coverage are implemented. |
| 05. MCP | Complete | RMCP stdio server exposes read-only tools, native resources/resource templates, and prompt templates. |
| 06. open_social_data bridge | Partial | Tabular adapter covers services, locations, organizations, codes, contacts, eligibility, and availability without hard dependency; initial data dictionaries exist. |
| 07. Live validation | Partial | UAT base URL, x-api-key auth, search parameter names, and a metadata-only live smoke are confirmed; broader pagination/nearby/direct-read/rate-limit captures remain metadata-only follow-ups. |
| 08. Release and distribution | Partial | Cargo.lock, RMCP pin, release runbook, binary release workflow, packaging docs, provenance plan, release checklist, and CI confirmation exist; public data, hosted proxy, redistribution, open-data, AI dataset, and non-research commercial use still need Healthpoint written approval. |

## Track 01 — Workspace foundation

Status: complete

Tasks:

- [x] Create Rust workspace.
- [x] Add core/client/fhir/cli/mcp/export/osd/testkit crates.
- [x] Add Conductor entrypoints.
- [x] Add docs and ADR skeletons.
- [x] Add CI, cargo-deny, rustfmt, clippy, and justfile scaffolding.
- [x] Add metadata-only Conductor setup for non-Rust sandboxes.
- [x] Add static preflight script for metadata/Conductor/fixture checks.
- [x] Add synthetic HTTP mock server for offline contract smoke tests.
- [x] Add interim generated JSON Schema contracts.
- [x] Add Conductor status command for handoff.
- [x] Validate JSON/TOML metadata in current environment.
- [x] Pass `git diff --check` in current environment.
- [x] Run `cargo fmt --all --check` in a Rust-enabled environment.
- [x] Run `cargo check --workspace --all-targets` in a Rust-enabled environment.
- [x] Run `cargo test --workspace` in a Rust-enabled environment.
- [x] Run `cargo clippy --workspace --all-targets -- -D warnings` in a Rust-enabled environment.

## Track 02 — Access and licensing boundary

Status: complete

Tasks:

- [x] Document bring-your-own-key stance.
- [x] Add export policy model.
- [x] Prohibit real response fixtures by convention.
- [x] Add same-origin guard for absolute pagination cursors.
- [x] Add FHIR id validation before resource URL construction.
- [x] Add literal redaction helper and redacted API errors.
- [x] Add local export manifest warnings.
- [x] Add conservative machine-readable policy stub.
- [x] Review Healthpoint API terms for caching, redistribution, attribution, and rate limits.
- [x] Convert terms review into an updated machine-readable policy file.

## Track 03 — FHIR mapping

Status: complete

Tasks:

- [x] Add synthetic HealthcareService Bundle fixture.
- [x] Add Location and Organization fixtures.
- [x] Map service id/name/provider/locations/type/specialty/telecom.
- [x] Map coverage areas, program, communication, referral methods, and appointment requirement.
- [x] Map identifiers, service provision codes, characteristics, eligibility, comments, extra details, availability, not-available, and endpoints.
- [x] Map Location address/position/managing organization.
- [x] Map Location identifiers, physical type, parent, endpoints, and hours.
- [x] Map Organization identifiers, type, aliases, parent, endpoints, and contacts.
- [x] Add Bundle pagination tests.
- [x] Add HTTP example responses derived from synthetic fixtures.
- [ ] Decide whether to adopt generated FHIR bindings or keep tolerant JSON mapping after live-profile validation.

## Track 04 — CLI

Status: partial

Tasks:

- [x] Add `doctor`.
- [x] Add `policy show`.
- [x] Add `search services`.
- [x] Add SNOMED convenience search.
- [x] Add nearby-search flags.
- [x] Add `get service`, `get organization`, `get location`, and `get uri`.
- [x] Add `inspect search-url` and `inspect resource-url`.
- [x] Add `export manifest`.
- [x] Add `export services` with manifest sidecar.
- [x] Add JSONL and CSV service output.
- [x] Add synthetic `fixture` commands.
- [x] Add JSON Schema emission.
- [x] Add checked-in interim schemas for non-Rust integration planning.
- [x] Add mock-server docs and smoke workflow.
- [x] Add mock-server integration tests after Cargo validation.
- [x] Add shell completions.

## Track 05 — MCP

Status: complete

Tasks:

- [x] Add rmcp-based stdio server crate.
- [x] Add read-only tool skeletons.
- [x] Add diagnostic/status, SNOMED, nearby, service, location, and organization tools.
- [x] Add explicit `healthpoint_read_resource_uri` tool as a safe bridge to future resources.
- [x] Add native MCP resource templates after RMCP API compile validation.
- [x] Add MCP prompt templates.
- [x] Pin rmcp dependency once the selected API surface is validated.

## Track 06 — open_social_data bridge

Status: partial

Tasks:

- [x] Add tabular view adapter crate without hard dependency.
- [x] Add service/location/organization/code/contact row views.
- [x] Add eligibility and availability row views.
- [x] Add initial view data dictionaries.
- [ ] Add parquet/Arrow export once local CSV/JSONL is proven.
- [ ] Add `open_social_data` provider integration only when licence review allows.

## Track 07 — Live validation

Status: partial

Tasks:

- [ ] Confirm base URL and auth scheme.
- [ ] Confirm search parameter names.
- [ ] Confirm pagination shape.
- [ ] Confirm nearby-search encoding.
- [ ] Confirm direct reads for HealthcareService, Location, and Organization.
- [ ] Confirm error/status/rate-limit response headers.
- [x] Add redacted live contract capture plan.
- [ ] Record only metadata and endpoint-shape notes; never commit real Healthpoint payloads.

## Track 08 — Release and distribution

Status: planned

Tasks:

- [x] Generate `Cargo.lock` in a Rust-enabled environment.
- [x] Decide whether to pin `rmcp` to crates.io release, tag, or commit hash.
- [x] Add binary release workflow.
- [x] Add packaging docs for MCP clients.
- [x] Add release provenance/attestation plan.
- [x] Add release runbook.
- [x] Add issue/PR templates with safety and licensing gates.
- [x] Add RMCP pin plan.
