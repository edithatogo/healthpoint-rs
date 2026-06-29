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

## Remaining blockers

- PR `#22` is open because branch protection requires at least one approving review from a reviewer with write access and required checks on the latest head.
- Crates.io publication is blocked until `CARGO_REGISTRY_TOKEN` is replaced/re-saved with a valid crates.io API token.
- Official MCP registry publication is blocked until the Cargo package/version exists on crates.io.
- Third-party MCP directory submissions remain account-gated or packaging-gated.
- Healthpoint written approval remains required before any public-data, production-hosted, redistribution, or open_social_data release.

## Next actions

1. Approve and merge PR `#22`.
2. Replace the GitHub Actions `CARGO_REGISTRY_TOKEN` secret with a valid crates.io publish token.
3. Rerun the release workflow with `publish_crates=true`.
4. Run `~/.local/bin/mcp-publisher login github` and publish to the official MCP registry after crates.io publication succeeds.
5. Submit third-party registry entries after official package publication is externally visible.
