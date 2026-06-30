# Track 08 — Release and distribution

Status: partial

Open tasks:

- Confirm GitHub CI passes after push, then treat any remaining publication items as external gates.

Implemented prep:

- Cargo.lock generated in a Rust-enabled environment.
- RMCP pinned to validated commit `67a30859443ab0fe79f2d50307c7d7bc9518f7e3`.
- Binary release workflow.
- MCP client packaging docs.
- Release provenance plan.
- Release runbook.
- RMCP pin plan.
- GitHub PR and issue templates.
- Documentation link-check workflow.

External gates:

- `CARGO_REGISTRY_TOKEN` repository secret.
- crates.io publication.
- Official MCP Registry submission after crates.io visibility.
- Smithery/Glama/PulseMCP/mcp.so publication or refresh where those services require account-side actions.
