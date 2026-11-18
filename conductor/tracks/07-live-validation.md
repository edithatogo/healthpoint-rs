# Track 07 — Live validation

Status: partial

Repo-local evidence already recorded:

- UAT base URL and `x-api-key` auth scheme are documented in `docs/healthpoint-api-access.md` and `docs/api-assumptions.md`.
- Observed query shapes and nearby encoding are documented in `docs/healthpoint-api-access.md`, `docs/healthpoint-portal-reference.md`, and `docs/api-assumptions.md`.
- Redacted capture guidance lives in `docs/live-contract-capture.md`.

Still live-only:

- Pagination shape and next-link/cursor behaviour.
- Live `Organization` read confirmation.
- Error/status/rate-limit response headers.

Rules:

- Use Dylan's API key locally only.
- Do not commit real Healthpoint payloads.
- Record endpoint-shape metadata only.
- Redact headers, tokens, and query strings if they reveal credentials.

Checklist lives in `docs/live-validation.md`; redacted contract capture plan lives in `docs/live-contract-capture.md`.

## Serde/API boundary assessment — 18 November 2026

The workspace already uses `serde` derive and `serde_json` for typed API/FHIR payloads; `chrono`, `uuid`, CSV and HTTP clients provide independent type/format/transport semantics. This is appropriate. **Do not replace** transport, FHIR mapping or CSV parsing with Serde traits. Provider-specific unknown fields and declared FHIR extensions require explicit preservation or logged loss, not blanket `deny_unknown_fields`.

- [ ] On the existing live-validation track, reconcile typed DTOs with current source provider fixtures for HealthcareService, Organization, Location, pagination and nested extension/value types; freeze exact raw-to-normalized fixtures.
- [ ] Add negative fixtures for duplicate keys, absent/unknown identifiers, Unicode, oversized bodies, date/time precision, versions and redacted errors. Preserve data licence, API secrets and existing live-only gates.
- [ ] Evaluate repeated manual JSON conversion only when a real call site is identified. Reuse typed Serde DTOs for stable exported contracts; do not harden dynamic provider extensions to closed types without lossless fallback.
- [ ] No claimed live validation, schema expansion or dependency migration until exact-head locked Rust/CLI/MCP/export tests and redacted UAT evidence are independently reviewed.
