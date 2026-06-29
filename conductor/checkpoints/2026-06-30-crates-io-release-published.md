# Crates.io release published

Date: 2026-06-30

## Completed

- Retrieved a crates.io publish token through the authenticated browser session without printing the token.
- Stored the token locally under ignored `.healthpoint/secrets/` and configured `.env.local` to point Cargo at that token file.
- Installed the token as the GitHub Actions `CARGO_REGISTRY_TOKEN` repository secret.
- Fixed release workflow attestation permissions.
- Fixed crates.io publish order and made publish reruns tolerate already-uploaded crate versions.
- Published GitHub release `v0.1.0` with Linux, macOS, and Windows artifacts plus checksum sidecars.
- Published all workspace crates to crates.io at `0.1.0`:
  - `healthpoint-core`
  - `healthpoint-fhir`
  - `healthpoint-testkit`
  - `healthpoint-client`
  - `healthpoint-export`
  - `healthpoint-osd-adapter`
  - `healthpoint-mcp`
  - `healthpoint-cli`
- Closed GitHub issues #20 and #21 with release and registry-readiness status.

## Evidence

- Release workflow: https://github.com/edithatogo/healthpoint-rs/actions/runs/28410068443
- GitHub release: https://github.com/edithatogo/healthpoint-rs/releases/tag/v0.1.0
- `cargo search healthpoint --limit 20` shows all eight workspace crates at `0.1.0`.

## Remaining gates

- Official MCP registry submission is blocked by local `mcp-publisher` authentication: `mcp-publisher publish server.json` reports `not authenticated`.
- Third-party MCP directories remain account-gated or require registry-side indexing/review.
- Healthpoint written approval is still required before production use, hosted proxying, public data redistribution, AI dataset use, or non-research commercial use.
