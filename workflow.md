# Conductor Development Workflow

## Delivery strategy

Develop `healthpoint-rs` in narrow, independently useful phases. Each phase must end with:

- a working command or library surface,
- tests or synthetic fixture checks,
- updated usage notes,
- updated Conductor state/tracks,
- explicit notes about access/licensing boundaries.

The default path is read-only, local-only, and bring-your-own-key. Any command that writes derived data must attach provenance and default to `local-only` export policy.

## Phase 1: Repository foundation and domain model

### Milestone 1.1: Rust workspace scaffold

- Create workspace crates for core, FHIR, client, CLI, MCP, export, OSD adapter, and testkit.
- Add Conductor setup/run entrypoints.
- Add CI, linting, formatter, security, and docs scaffolding.
- Acceptance: `cargo check --workspace --all-targets` succeeds.

### Milestone 1.2: FHIR-first record model

- Map synthetic `HealthcareService` Bundle to `ServiceRecord`.
- Preserve raw FHIR JSON on typed records.
- Add `SourceProvenance` and `AccessPolicy` to every record/page/export.
- Acceptance: tests map synthetic fixtures without real Healthpoint data.

## Phase 2: Licensed API client and CLI

### Milestone 2.1: Read-only client

- Implement configurable base URL, auth scheme, and conservative request limits.
- Support search/get for `HealthcareService` and get for `Organization` and `Location`.
- Acceptance: mock-server tests pass; live tests remain ignored and key-gated.

### Milestone 2.2: CLI surface

- Add `doctor`, `search services`, `get service`, `get organization`, `get location`, and `export manifest`.
- Add JSON, JSONL, CSV, and human output modes where appropriate.
- Acceptance: CLI never prints secrets and all errors include remediation hints.

## Phase 3: MCP server

### Milestone 3.1: Read-only MCP tools

- Expose diagnostic, service search, SNOMED search, nearby search, service get, location get, and organization get tools.
- Add clear tool descriptions, schemas, limits, and licence warnings.
- Acceptance: server launches over stdio and tool calls return provenance-rich JSON.

### Milestone 3.2: MCP resources and prompts

- Add resource templates for `healthpoint://service/{id}`, `healthpoint://organization/{id}`, and `healthpoint://location/{id}`.
- Add prompts for safe directory summaries and code-mapping explanations.
- Acceptance: resources are read-only and paginated where relevant.

## Phase 4: Export and integration layer

### Milestone 4.1: Local export views

- Add JSONL, CSV, and later Parquet/Arrow exports.
- Add manifest sidecars for every export.
- Acceptance: all exports include retrieved-at, source, access mode, and redistribution status.

### Milestone 4.2: open_social_data bridge

- Stabilise tabular views before adding a hard dependency.
- Add adapter that can be consumed by `open_social_data` only when terms permit.
- Acceptance: no command labels Healthpoint-derived outputs as open without `open-approved` policy.

## Cross-cutting requirements

- Respect Healthpoint terms, rate limits, and user-owned API key boundaries.
- Do not scrape, bypass access controls, proxy keys, or publish caches by default.
- Keep real API responses out of Git.
- Redact secrets from logs, traces, tests, errors, and diagnostics.
- Prefer synthetic fixtures for CI.
- Treat the `rmcp` dependency as bleeding-edge; pin before stable releases.
