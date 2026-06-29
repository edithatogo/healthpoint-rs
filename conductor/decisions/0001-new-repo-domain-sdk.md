# ADR 0001: Create `healthpoint-rs` as the canonical repo

## Status

Accepted.

## Context

Healthpoint is a distinct licensed, real-time health-service directory API. Existing repos provide useful patterns but do not own this domain.

## Decision

Create a new Rust workspace named `healthpoint-rs`.

## Consequences

- Healthpoint-specific API and FHIR logic lives in one canonical repo.
- Existing tools can integrate later through stable adapter crates.
- The repo can enforce its own licensing, provenance, and MCP security model.
