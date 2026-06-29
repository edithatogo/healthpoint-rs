# ADR 0005: Use bleeding-edge `rmcp` initially

## Status

Experimental.

## Context

The official Rust MCP SDK is moving quickly. The project goal is frontier MCP capability, but release builds need reproducibility.

## Decision

Use the official `modelcontextprotocol/rust-sdk` `main` branch in the initial scaffold. Pin to a release/tag before any public binary release.

## Consequences

- The MCP crate can track current SDK patterns early.
- CI may break when upstream changes; this is acceptable during spike/scaffold phase.
- A dependency-pinning milestone is required before release.
