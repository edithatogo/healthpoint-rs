# ADR 0005: Pin `rmcp` after initial validation

## Status

Accepted.

## Context

The official Rust MCP SDK is moving quickly. The project goal is frontier MCP capability, but release builds need reproducibility.

## Decision

The initial scaffold used the official `modelcontextprotocol/rust-sdk` `main` branch. After local compile/test/clippy validation, pin the dependency to commit `67a30859443ab0fe79f2d50307c7d7bc9518f7e3` with the `server` and `transport-io` features.

Prefer a crates.io release or upstream tag later when the required stdio server API is available through a stable release artifact.

## Consequences

- Local and CI builds are reproducible against the validated RMCP API surface.
- The MCP crate no longer floats with upstream `main`.
- A later dependency refresh requires explicit validation and an updated ADR/checkpoint.
