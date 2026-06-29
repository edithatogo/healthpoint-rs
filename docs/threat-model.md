# Threat model

## Assets

- Dylan's Healthpoint API key and any other licensed credentials.
- Healthpoint-origin response bodies and derived exports.
- Local caches, manifests, logs, traces, shell history, and MCP client configuration.
- Downstream open_social_data dataset packs if ever enabled.

## Primary risks

| Risk | Mitigation |
| --- | --- |
| API key leakage in logs or config | `hide_env_values`, redacted diagnostics, secret scan, `.env` ignored, server manifest marks API key secret. |
| Accidental redistribution of licensed data | local-only export policy, manifest warnings, no real fixtures, open-data mode disabled until terms review. |
| Bulk mirror behaviour | conservative `_count` clamp, no dump-all command, explicit export command with sidecar manifest. |
| Cursor exfiltration or SSRF-like following | same-origin pagination cursor guard before absolute URL reuse. |
| Path injection via resource IDs | FHIR id validation before path-segment construction. |
| Overclaiming API compatibility | synthetic mock server clearly labelled, live contract capture requires redaction and manual validation. |
| MCP client overuse | read-only tools, limit clamping, no write tools, no public proxy mode. |

## Out of scope

This tool is not clinical decision support. It does not assess service quality, eligibility correctness, clinical urgency, referral appropriateness, or patient-specific advice.

## Required before public release

- First successful Rust CI run.
- Confirmed Healthpoint terms for caching, attribution, rate limits, and redistribution.
- `rmcp` dependency pinned to a crates.io version, tag, or commit SHA.
- Gitleaks pass on full history.
- Live contract capture converted into redacted documentation.
