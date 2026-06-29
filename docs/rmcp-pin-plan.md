# RMCP dependency pin plan

The workspace currently follows the `modelcontextprotocol/rust-sdk` `main` branch for frontier MCP features. That is intentional during the spike phase, but it is not a release posture.

Before publishing binaries or crates:

1. Run `cargo check -p healthpoint-mcp --all-targets`.
2. Decide whether the used macros/API match a crates.io release.
3. Prefer a crates.io version when possible.
4. If crates.io is behind required functionality, pin to a commit SHA rather than `branch = "main"`.
5. Record the decision in a new ADR and `conductor/state.json`.
6. Regenerate `Cargo.lock` and commit it.

The MCP server must remain read-only regardless of RMCP transport/capability changes.
