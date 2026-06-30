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
