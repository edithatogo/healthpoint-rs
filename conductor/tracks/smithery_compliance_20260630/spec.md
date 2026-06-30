# Smithery Compliance Hardening with CLI MCP Validation

## Overview

Maximize free, repo-controlled Smithery compliance for `healthpoint-rs` and create a reusable Smithery compliance pattern for other MCP repos. The current Smithery score is `51/100`; paid plan and exact-host DNS TXT verification remain external gates.

## Functional requirements

- Use `RHEcosystemAppEng/mcp-validation` as the primary CLI validator path.
- Keep `Janix-ai/mcp-validator` as reference-only because it is AGPL-licensed.
- Keep Apify `rocketagro/mcp-validator` as optional/manual because it is a paid cloud Actor.
- Treat `punkpeye/awesome-mcp-servers` as a directory/readme benchmark, not a validator.
- Add local scripts for MCP protocol validation and static Smithery/MCPB compliance checks.
- Improve MCPB metadata so declared tools, prompts, resources, and templates match the actual MCP surface.
- Add multi-platform CI for Linux, macOS, and Windows MCPB bundles.
- Publish to Smithery only when explicitly requested and `SMITHERY_TOKEN` is configured.
- Add a reusable Smithery compliance playbook for other repos.

## Non-functional requirements

- Do not bundle Healthpoint API keys, `.env` files, real API payloads, exports, logs, traces, databases, or generated reports.
- Keep Healthpoint access BYO-key and read-only.
- Do not add hosted Streamable HTTP proxying in this track.
- Keep paid plan and DNS TXT verification out of implementation scope unless separately approved.

## Acceptance criteria

- `scripts/package-mcpb` builds a bundle from a metadata source of truth.
- `scripts/check-smithery-compliance.py` passes locally after bundle generation.
- `scripts/validate-mcp-server fast` produces an ignored JSON report when `mcp-validate` is installed.
- GitHub Actions can build MCPB bundles on Linux, macOS, and Windows.
- Docs record validator choices, score state, external gates, and reusable cross-repo guidance.
- Conductor state/checkpoints/tracks reflect the new compliance hardening lane.

## Out of scope

- Paid Smithery developer plan purchase.
- DNS TXT setup on a controlled homepage host.
- Hosted public MCP proxy or Streamable HTTP endpoint.
- Public Healthpoint data redistribution or AI dataset use.
