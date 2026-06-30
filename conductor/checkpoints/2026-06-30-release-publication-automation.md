# Release publication automation checkpoint

Date: 2026-06-30

## Completed

- Centralized package version at `workspace.package.version = "0.1.0"`.
- Updated workspace crates to inherit `version.workspace = true`.
- Added path plus version metadata for internal workspace dependencies so crates can publish in dependency order.
- Switched `rmcp` from the validated Git revision to crates.io `rmcp = "2.0.0"`.
- Added `scripts/check-release-version.py` for tag/server/changelog/package version consistency.
- Added `scripts/check-mcp-registry-submission.py` for MCP registry readiness checks and registry matrix output.
- Expanded `server.json` with release, package, repository, licence, homepage, and keyword metadata.
- Replaced the release workflow with a tag-aware workflow that validates release metadata, builds cross-platform artifacts, uploads checksums, creates a GitHub release, and publishes crates when `CARGO_REGISTRY_TOKEN` exists.
- Added `.github/workflows/mcp-registry-submission.yml` to verify registry requirements and gate official MCP submission.
- Added `docs/mcp-registry-submissions.md`.

## Local validation

- `scripts/check-release-version.py --tag v0.1.0` passed.
- `scripts/check-mcp-registry-submission.py` passed.
- `scripts/static-preflight.py` passed.
- `cargo fmt --all --check` passed.
- `cargo check --workspace --all-targets --locked` passed.
- `cargo test --workspace --locked` passed.
- `cargo clippy --workspace --all-targets --locked -- -D warnings` passed.
- `cargo deny check` passed with duplicate-version warnings only.
- `cargo publish -p healthpoint-core --dry-run --allow-dirty --locked` passed.
- `cargo package --list` passed for dependent publish crates.

## Remaining gates

- `CARGO_REGISTRY_TOKEN` is not present locally or as a repository secret, so crates.io publication cannot be completed yet.
- Official MCP registry submission requires the crates.io packages to exist and `mcp-publisher` credentials/tooling.
- Smithery, Glama, PulseMCP, and mcp.so submissions are account-gated/manual unless those services expose usable credentials or APIs.
- Healthpoint written approval is still required before public data redistribution, open data publication, hosted proxy use, AI dataset/model training use, or non-research commercial use.
- All remaining gates above are external to the repository.
