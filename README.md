# healthpoint-rs

Rust-first tooling for the Healthpoint HL7 FHIR® API: a typed client, CLI, MCP server, and future open-social-data adapter.

This repository is intentionally **code-first and data-light**:

- Users bring their own Healthpoint API key/licence.
- No real Healthpoint API payloads are committed as fixtures.
- Synthetic FHIR fixtures live in `crates/healthpoint-testkit/fixtures/`.
- Local exports are marked with provenance and redistribution status.
- Open-data publication is opt-in and disabled until licensing/access terms explicitly permit it.

## Project shape

```text
healthpoint-rs/
  crates/
    healthpoint-core/         # domain model, query model, provenance, provider traits
    healthpoint-fhir/         # FHIR Bundle/Resource mapping, HealthcareService extraction
    healthpoint-client/       # HTTP client, auth, request policy, pagination hooks
    healthpoint-export/       # JSON/JSONL/CSV/Markdown/export manifests
    healthpoint-cli/          # `healthpoint` CLI
    healthpoint-mcp/          # read-only MCP server over the same core/client
    healthpoint-osd-adapter/  # future open_social_data bridge, no hard dependency yet
    healthpoint-testkit/      # synthetic fixtures and test helpers
  conductor/                  # context-management state, decisions, and implementation tracks
  docs/                       # access, licensing, MCP, exports, integration roadmap
```

## First commands

```bash
cp .env.example .env
$EDITOR .env
bin/conductor-setup
cargo run -p healthpoint-cli -- doctor
cargo run -p healthpoint-cli -- search services --text "cervical screening" --format json
cargo run -p healthpoint-cli -- get service <id> --format json
cargo run -p healthpoint-mcp
```

The MCP server is a separate binary so CLI and MCP can evolve independently while sharing the same crates.

## Configuration

```bash
export HEALTHPOINT_API_KEY="..."
export HEALTHPOINT_BASE_URL="https://www.healthpointapi.com/"
export HEALTHPOINT_AUTH_SCHEME="bearer"        # bearer | x-api-key | header:<name> | none
export HEALTHPOINT_EXPORT_POLICY="local-only"  # local-only | licensed-share | open-approved
```

The default assumes bearer-token auth because the public API landing page does not expose full developer authentication details. Use `HEALTHPOINT_AUTH_SCHEME=x-api-key` or `header:<name>` if your Healthpoint credentials require a named API-key header.

## Design principles

1. **FHIR-first**: preserve raw FHIR while exposing typed domain records.
2. **Read-only by default**: no writes, no scraping fallback, no public proxy mode.
3. **Bring-your-own-key**: releasing code does not bundle access or data rights.
4. **Provenance everywhere**: exports carry retrieval time, source, licence status, and tool version.
5. **Integratable later**: stable Rust traits make it possible to plug into `open_social_data`, MCP clients, and future data/catalog engines.
6. **Conductor-managed context**: implementation tracks, decisions, and repo state live alongside the code.

## Safety boundary

This is not a clinical decision-support system. It retrieves and formats directory/service information from Healthpoint for licensed users. Any downstream use should preserve Healthpoint attribution, currency, caveats, and licensing obligations.

## Current status

Initial scaffold. The public Healthpoint material confirms HL7 FHIR and SNOMED CT orientation, but full endpoint/auth details are intentionally treated as configurable until validated against licensed API documentation.
