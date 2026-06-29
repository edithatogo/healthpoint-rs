# MCP registry readiness

This project is not ready for registry submission until the Rust workspace compiles and the RMCP dependency is pinned.

## Current manifest

`server.json` declares a stdio server named `healthpoint-rs` and marks `HEALTHPOINT_API_KEY` as a secret. It assumes a locally installed `healthpoint-mcp` binary.

## Before submission

- [ ] Compile `healthpoint-mcp` on Linux, macOS, and Windows.
- [ ] Pin `rmcp` to a crates.io version, tag, or commit SHA.
- [ ] Confirm MCP tool schemas from the generated RMCP output.
- [ ] Add native MCP resources or explicitly document tools-only status.
- [ ] Confirm no tool returns API keys, raw credentials, or unredacted error bodies.
- [ ] Run the server through at least one MCP client using the synthetic mock server.
- [ ] Re-check Healthpoint terms for allowed distribution of a generic BYO-key client.

## Read-only claim

All current MCP tools are read-only. Any future write-like operation must require a new ADR, explicit user confirmation, and a revised access-policy document.
