# healthpoint-rs Conductor Brief

## Mission

Build a Rust-first Healthpoint API toolkit that provides a typed SDK, CLI, MCP server, and future data-plane adapter without bundling or redistributing licensed Healthpoint data.

## Architectural stance

- New canonical repo: `healthpoint-rs`.
- `substack-cli-ts` is a workflow reference only.
- `open_social_data` is a future integration target for approved tabular exports, not the initial core.
- Healthpoint data access is bring-your-own-key.
- FHIR resources remain the canonical internal representation; tables are views.

## Current priorities

1. Validate API auth/base URL from licensed documentation.
2. Confirm search parameter names for text, SNOMED/type/specialty, and nearby search.
3. Make synthetic fixture mapping compile and pass.
4. Make `doctor` and JSON search/get commands robust.
5. Add MCP tools over the same client.

## Context-packing rule

Every future agent should read, in order:

1. `conductor/state.json`
2. `conductor/brief.md`
3. `conductor/tracks.md`
4. latest ADRs in `conductor/decisions/`
5. `docs/access-and-licensing.md`
6. `docs/api-assumptions.md`
