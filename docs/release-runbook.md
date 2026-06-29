# Release runbook

## Release prerequisites

- Rust CI is green on `main`.
- `Cargo.lock` is generated and committed.
- `rmcp` is pinned away from `branch = "main"`.
- `cargo deny check` is green.
- Gitleaks is green on full history.
- Live contract documentation is redacted and current.
- Healthpoint access/caching/redistribution terms are reviewed.
- No real Healthpoint response bodies are in the repository.

## Dry run

```bash
scripts/static-preflight.py
scripts/generate-contract-schemas.py
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check
```

## Tagging

Do not tag a public release until the MCP server has been tested with a real installed binary and at least one MCP client configuration.

Suggested tag format:

```text
v0.1.0-alpha.1
```

## Post-release checks

- Install binary into a clean environment.
- Run `healthpoint doctor`.
- Run `healthpoint-mcp` under a stdio MCP client using the synthetic mock server.
- Confirm `server.json` remains accurate.
