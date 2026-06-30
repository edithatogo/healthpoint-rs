# healthpoint-rs

[![smithery badge](https://smithery.ai/badge/edithatogo/healthpoint-rs)](https://smithery.ai/servers/edithatogo/healthpoint-rs)

Rust-first tooling for the Healthpoint HL7 FHIR® API: a typed client, CLI, read-only MCP server, and future open-social-data adapter.

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
    healthpoint-core/         # domain model, query model, provenance, provider traits, URI parsing
    healthpoint-fhir/         # FHIR Bundle/Resource mapping and typed projections
    healthpoint-client/       # HTTP client, auth, request policy, pagination hooks
    healthpoint-export/       # JSON/JSONL/CSV/export manifests
    healthpoint-cli/          # `healthpoint` CLI
    healthpoint-mcp/          # read-only MCP server over the same core/client
    healthpoint-osd-adapter/  # future open_social_data bridge, no hard dependency yet
    healthpoint-testkit/      # synthetic fixtures and offline fixture provider
  conductor/                  # context-management state, decisions, checkpoints, tracks
  docs/                       # access, licensing, MCP, exports, integration roadmap
```

## Installation

Install from crates.io after Rust is available:

```bash
cargo install healthpoint-mcp healthpoint-cli
```

Install from source checkout:

```bash
git clone https://github.com/edithatogo/healthpoint-rs.git
cd healthpoint-rs
bin/conductor-setup
cargo install --path crates/healthpoint-mcp
cargo install --path crates/healthpoint-cli
```

Install through Smithery from the published listing: [edithatogo/healthpoint-rs](https://smithery.ai/servers/edithatogo/healthpoint-rs). The Smithery package starts in synthetic mode unless live Healthpoint credentials are supplied.

## Usage

First commands

```bash
cp .env.example .env
$EDITOR .env
bin/conductor-setup
cargo run -p healthpoint-cli -- doctor
cargo run -p healthpoint-cli -- fixture services --format json
cargo run -p healthpoint-cli -- inspect search-url --text "cervical screening" --snomed 171149006
cargo run -p healthpoint-cli -- search services --text "cervical screening" --format json
cargo run -p healthpoint-cli -- search services --snomed 171149006 --format json
cargo run -p healthpoint-cli -- get service <id> --format json
cargo run -p healthpoint-cli -- get uri healthpoint://service/<id> --format json
cargo run -p healthpoint-mcp
```

The MCP server is a separate binary so CLI and MCP can evolve independently while sharing the same crates. It starts in synthetic fixture mode when no API key is supplied; set `HEALTHPOINT_MODE=live` and provide `HEALTHPOINT_API_KEY` for licensed live API calls.

## Configuration

```bash
export HEALTHPOINT_MODE="synthetic"                       # synthetic | live
export HEALTHPOINT_API_KEY="..."                         # optional; required only for live mode
export HEALTHPOINT_BASE_URL="https://uat.healthpointapi.com/baseR4/"
export HEALTHPOINT_AUTH_SCHEME="x-api-key"               # bearer | x-api-key | header:<name> | none
export HEALTHPOINT_GEO_SEARCH_MODE="healthpoint-lat-lon" # healthpoint-lat-lon | fhir-near
export HEALTHPOINT_TIMEOUT_SECS="30"
export HEALTHPOINT_EXPORT_POLICY="local-only"            # local-only | licensed-share | open-approved
```

Healthpoint portal validation on 2026-06-30 confirmed UAT calls use the `x-api-key` header against `https://uat.healthpointapi.com/baseR4/`. See `docs/healthpoint-api-access.md` for observed endpoint and license notes.

## CLI examples

```bash
healthpoint doctor
healthpoint policy show

healthpoint fixture services --format human
healthpoint schema service-record
healthpoint schema resource-uri

healthpoint inspect search-url \
  --text "cervical screening" \
  --snomed 171149006 \
  --limit 10

healthpoint search services \
  --text "cervical screening" \
  --snomed 171149006 \
  --limit 10 \
  --format json

healthpoint search services \
  --lat -36.8485 \
  --lon 174.7633 \
  --radius-km 10 \
  --format csv

healthpoint get service <service-id> --format json
healthpoint get location <location-id> --format json
healthpoint get organization <organization-id> --format json
healthpoint get uri healthpoint://service/<service-id> --format json

healthpoint export manifest --output .healthpoint/manifest.json
healthpoint export services \
  --text "cervical screening" \
  --limit 25 \
  --format jsonl \
  --output .healthpoint/cervical-screening.jsonl
```

## Tools

| Tool | Purpose |
| --- | --- |
| `healthpoint.diagnostic.status` | Show redacted runtime mode, configuration, and readiness. |
| `healthpoint.access.notes` | Show non-secret endpoint, auth, and documentation notes. |
| `healthpoint.access.policy` | Show the conservative access/export policy before reuse. |
| `healthpoint.services.search` | Search HealthcareService records by text, codes, region filters, cursor, and limit. |
| `healthpoint.services.search_snomed` | Search HealthcareService records by SNOMED CT code in type, category, or specialty. |
| `healthpoint.services.nearby` | Find HealthcareService records near a latitude/longitude point. |
| `healthpoint.service.get` | Read one HealthcareService by FHIR id. |
| `healthpoint.location.get` | Read one Location by FHIR id. |
| `healthpoint.organization.get` | Read one Organization by FHIR id. |
| `healthpoint.resource.read` | Read a supported `healthpoint://` resource URI. |

The MCP server also exposes 3 static resources, 4 resource templates, and 2 prompts. See `docs/mcp-tools.md` and `docs/integrations/mcp-client-configs.md` for launch examples.

Claude Desktop source-checkout example:

```json
{
  "mcpServers": {
    "healthpoint-dev": {
      "command": "cargo",
      "args": ["run", "-p", "healthpoint-mcp"],
      "env": {
        "HEALTHPOINT_MODE": "synthetic"
      }
    }
  }
}
```

Live Healthpoint mode requires a licensed API key:

```json
{
  "mcpServers": {
    "healthpoint-live": {
      "command": "healthpoint-mcp",
      "env": {
        "HEALTHPOINT_MODE": "live",
        "HEALTHPOINT_API_KEY": "...",
        "HEALTHPOINT_BASE_URL": "https://uat.healthpointapi.com/baseR4/",
        "HEALTHPOINT_AUTH_SCHEME": "x-api-key"
      }
    }
  }
}
```

## Integration contracts

The CLI can emit JSON Schema for the core contracts:

```bash
healthpoint schema access-policy
healthpoint schema service-query
healthpoint schema service-record
healthpoint schema service-page
healthpoint schema location-record
healthpoint schema organization-record
healthpoint schema resource-uri
healthpoint schema export-manifest
```

Those schemas are intended to help future integration with `open_social_data`, MCP clients, and any cross-repo data catalogue layer without prematurely forcing Healthpoint's FHIR graph into a dataframe-first shape.


## Offline readiness tools

These commands work before any live Healthpoint validation and are useful in sandboxes or CI metadata jobs:

```bash
CONDUCTOR_ALLOW_NO_CARGO=1 bin/conductor-setup
bin/conductor-status
scripts/static-preflight.py
scripts/generate-contract-schemas.py
bin/mock-healthpoint-server --port 8787
```

After Rust is available, the mock server gives the CLI a synthetic HTTP target:

```bash
export HEALTHPOINT_BASE_URL="http://127.0.0.1:8787/"
export HEALTHPOINT_AUTH_SCHEME="none"
cargo run -p healthpoint-cli -- search services --snomed 171149006 --format json
```

See `docs/mock-server.md`, `docs/static-preflight.md`, and `docs/live-contract-capture.md`.

## Development environment

See `docs/development-environment.md` for native Rust and devcontainer setup.

## Design principles

1. **FHIR-first**: preserve raw FHIR while exposing typed domain records.
2. **Read-only by default**: no writes, no scraping fallback, no public proxy mode.
3. **Bring-your-own-key**: releasing code does not bundle access or data rights.
4. **Provenance everywhere**: exports carry retrieval time, source, licence status, and tool version.
5. **Integratable later**: stable Rust traits make it possible to plug into `open_social_data`, MCP clients, and future data/catalog engines.
6. **Conductor-managed context**: implementation tracks, decisions, and repo state live alongside the code.
7. **Synthetic fixtures only**: offline testing is done with testkit resources, never real Healthpoint payloads.

## MCP Registry metadata

- MCP Registry name: `mcp-name: io.github.edithatogo/healthpoint-rs`

This visible marker is required for Cargo/crates.io ownership verification by the official MCP Registry.

## Safety boundary

This is not a clinical decision-support system. It retrieves and formats directory/service information from Healthpoint for licensed users. Any downstream use should preserve Healthpoint attribution, currency, caveats, and licensing obligations.

## Current status

Implementation spike after initial scaffold. Synthetic mapping exists for `HealthcareService`, `Location`, and `Organization`, including richer service fields such as eligibility, availability, service provision codes, characteristics, comments, endpoints, identifiers, and response provenance. The public Healthpoint material confirms HL7 FHIR and SNOMED CT orientation, but full endpoint/auth details are intentionally treated as configurable until validated against licensed API documentation.
