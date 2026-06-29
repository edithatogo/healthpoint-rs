# ADR 0007: Emit JSON Schema contracts from the CLI

Status: accepted

## Context

Future integration across MCP clients, open_social_data, and possible catalogues should not require every consumer to import Rust crates during the spike phase.

## Decision

Expose schemas through:

```bash
healthpoint schema <target>
```

Initial targets include access policy, service query, service/location/organization records, service pages, resource URIs, and export manifests.

## Consequences

- External adapters can validate payloads.
- Schema changes are visible in CLI output.
- Breaking schema changes should receive a Conductor decision record.
