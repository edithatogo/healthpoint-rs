# JSON Schema contracts

`healthpoint-rs` exposes JSON Schemas from the CLI so adapters can consume a stable contract without importing Rust crates.

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

These schemas are useful for:

- MCP clients that want to validate saved query payloads,
- `open_social_data` provider adapters,
- data catalogue metadata,
- contract tests around exported JSON/JSONL,
- future UI forms around Healthpoint search.

Schema stability rules:

1. Additive fields are allowed during the spike phase.
2. Renaming or removing fields requires a Conductor decision record.
3. Any field derived from licensed Healthpoint data must retain provenance and access-policy context.
