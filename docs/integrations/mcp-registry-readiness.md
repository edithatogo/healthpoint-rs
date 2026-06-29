# MCP registry readiness

This project is not ready for registry submission until the server is validated across release platforms and with at least one MCP client.

## Current manifest

`server.json` declares a stdio server named `healthpoint-rs` and marks `HEALTHPOINT_API_KEY` as a secret. It assumes a locally installed `healthpoint-mcp` binary.

## Before submission

- [ ] Compile `healthpoint-mcp` on Linux, macOS, and Windows.
- [x] Pin `rmcp` to a crates.io version, tag, or commit SHA.
- [x] Confirm MCP tool/resource/prompt schemas from the generated RMCP output.
- [x] Add native MCP resources or explicitly document tools-only status.
- [x] Confirm no tool returns API keys, raw credentials, or unredacted error bodies.
- [x] Run the server through at least one MCP client using the synthetic mock server.
- [x] Re-check Healthpoint terms for allowed distribution of a generic BYO-key client.

## Read-only claim

All current MCP tools are read-only. Any future write-like operation must require a new ADR, explicit user confirmation, and a revised access-policy document.
