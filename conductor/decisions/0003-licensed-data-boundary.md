# ADR 0003: Open code, licensed data

## Status

Accepted.

## Context

Users need Healthpoint API access. Publishing a client is different from publishing retrieved data.

## Decision

The repo is open-source code. Users bring their own API key/licence. Real retrieved data is local-only by default and not committed.

## Consequences

- The repository can be public without bundling licensed data.
- Exports must include provenance and redistribution status.
- Open dataset publication requires explicit terms review and `open-approved` policy.
