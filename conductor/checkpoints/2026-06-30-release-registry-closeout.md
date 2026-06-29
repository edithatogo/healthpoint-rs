# Release and registry closeout checkpoint

Date: 2026-06-30

## Completed

- Confirmed release workflow run `28407239423` validated release inputs, MCP registry readiness, Cargo package validation, Linux/macOS/Windows release binary builds, and GitHub Release update.
- Confirmed crates.io publication failed only at the publish step because `CARGO_REGISTRY_TOKEN` was empty/unusable in the workflow runtime.
- Enabled `main` branch protection with required status checks, one approving review, conversation resolution, stale-review dismissal, admin enforcement, and disabled force-push/delete behavior.
- Installed the official MCP publisher locally as `~/.local/bin/mcp-publisher` version `1.7.9`.
- Opened PR `#22` for release provenance and registry hardening.
- Added cargo metadata snapshots to release archives, GitHub artifact attestation workflow wiring, SBOM/attestation documentation, and `glama.json`.
- Addressed registry review feedback by requiring an explicit `HEALTHPOINT_BASE_URL` in Glama metadata instead of defaulting installations to UAT or production.
- Updated issues `#15`, `#16`, `#20`, and `#21` with current completion and blocker status.
- Confirmed all PR `#22` checks passed on latest head `f4034d0`.
- Enabled repository auto-merge and armed PR `#22` for squash auto-merge once a write-access approving review lands.
- Confirmed `healthpoint-mcp` and `healthpoint-core` are not yet externally visible on crates.io.
- Confirmed `mcp-publisher publish server.json` is blocked on MCP publisher authentication and should wait until crates.io publication succeeds.
- Resolved outdated review conversations on PR `#22`.
- Merged PR `#22` into `main`.
- Corrected branch protection required checks from workflow-level names to actual job/check names.

## Remaining blockers

- Crates.io publication is blocked until `CARGO_REGISTRY_TOKEN` is replaced/re-saved with a valid crates.io API token.
- Official MCP registry publication is blocked until the Cargo package/version exists on crates.io.
- Third-party MCP directory submissions remain account-gated or packaging-gated.
- Healthpoint written approval remains required before any public-data, production-hosted, redistribution, or open_social_data release.

## Next actions

1. Replace the GitHub Actions `CARGO_REGISTRY_TOKEN` secret with a valid crates.io publish token.
2. Rerun the release workflow with `publish_crates=true`.
3. Run `~/.local/bin/mcp-publisher login github` and publish to the official MCP registry after crates.io publication succeeds.
4. Submit third-party registry entries after official package publication is externally visible.
