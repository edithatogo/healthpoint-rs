# Smithery 100/100 remediation checkpoint

Date: 2026-06-30

## Implemented

- Added explicit MCP runtime identity and instructions so validators see `healthpoint-rs v0.1.0` instead of generic RMCP identity.
- Renamed the Smithery-facing MCP tool surface to dot notation.
- Added read-only tool annotations, parameter descriptions, and descriptive output schemas to MCPB metadata.
- Added optional no-credential synthetic mode and made live Healthpoint access opt-in/BYO-key.
- Added root `smithery.yaml` with stdio start command and optional compact config schema.
- Added a repo icon asset and stricter local Smithery compliance checker for Capability Quality, Server Metadata, and Configuration UX.
- Updated README, MCP docs, packaging docs, registry docs, and compliance contract.

## Validation

- `cargo fmt --all && scripts/package-mcpb` passed and produced `target/mcpb/healthpoint-rs-0.1.0-darwin-arm64.mcpb`.
- `scripts/check-smithery-compliance.py` passed.
- `scripts/validate-mcp-server fast` passed and reported `healthpoint-rs v0.1.0`, 10 tools, 2 prompts, and 3 resources.
- `smithery mcp publish target/mcpb/healthpoint-rs-0.1.0-darwin-arm64.mcpb -n edithatogo/healthpoint-rs` was accepted as deployment `677ad41a-384f-40d6-9ce0-b02774bb7294`.
- Public Smithery API recheck showed `configSchema.required` is now empty.

## Remaining gates

- Public Smithery dashboard currently shows `98/100`, but the final `2` points are not attributable from public HTML/API responses in this shell.
- Public Smithery API now reflects the reachable icon and optional configuration; any remaining listing-field persistence for `homepage`, `repositoryUrl`, `backlinkUrl`, `license`, or `unlisted` remains an account-session/dashboard gate.
- Exact-host DNS TXT verification and any paid developer-plan verification remain external gates.

## Follow-up loop on 90/100 score

- Added explicit runtime `output_schema` attributes to all 10 RMCP tool declarations.
- Verified direct MCP `tools/list` now returns non-null output schemas and annotations for all tools.
- Published Smithery release `36ec9477-599d-4d76-a900-f37d83753746`.
- Upgraded runtime output schemas to include explicit `properties` and changed icon metadata to a reachable SVG URL.
- Published Smithery release `dda10c87-9df1-4f17-9f4d-0b9bb5c950ba`.
- Authenticated Smithery API confirms the active bundle URL points at `dda10c87-9df1-4f17-9f4d-0b9bb5c950ba`, `required` config is empty, 10 tools have `outputSchema`, and `iconUrl` is `https://github.githubassets.com/favicons/favicon.svg`.
- The remaining uncertainty is the dashboard-only numeric quality score delta; public HTML/API responses did not expose the incomplete sub-item.
