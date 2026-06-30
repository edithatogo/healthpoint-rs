# Smithery compliance hardening

Date: 2026-06-30

## Completed in this pass

- Added Conductor track `smithery_compliance_20260630`.
- Added MCPB metadata source `packaging/mcpb/manifest-metadata.json`.
- Refactored `scripts/package-mcpb` to generate the bundle manifest from metadata.
- Added `scripts/check-smithery-compliance.py` for static bundle and metadata checks.
- Added `scripts/validate-mcp-server` for the preferred `mcp-validation` CLI.
- Added `.github/workflows/smithery-compliance.yml` for MCP validation and multi-platform MCPB bundle builds.
- Added reusable playbook `docs/smithery-compliance-playbook.md`.
- Updated registry and packaging docs with validator choices and external gate separation.

## External gates

- Smithery exact-host TXT verification needs a controlled homepage host.
- Smithery paid developer plan needs explicit user approval.
- Healthpoint written approval remains required before hosted proxying or public data redistribution.

## Captured evidence

- `scripts/package-mcpb` generated `target/mcpb/healthpoint-rs-0.1.0-darwin-arm64.mcpb`.
- `scripts/check-smithery-compliance.py` passed.
- `scripts/validate-mcp-server fast` passed with `mcp-validation` installed from GitHub in an isolated Python 3.13 venv.
- Validator report: `target/mcp-validation/healthpoint-mcp-validation.json`.
- The validator reported valid MCP protocol compliance, 10 tools, 2 prompts, 3 resources, ping support, error compliance, repository availability, license, and registry checks.
- Smithery accepted hardened release `76edfcf1-d617-42fe-8b4e-06c6e3917854`.

## Next evidence to capture
- Accepted Smithery deployment ID after publishing the hardened bundle.
- Updated Smithery score after the hardened release is visible.

## Pending browser evidence

- Smithery score re-check after hardened release is pending because Chrome timed out while loading the verification page.
