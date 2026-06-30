# ADR 0009: Keep the open_social_data bridge string-row based until provider terms are settled

## Status

Accepted.

## Context

`healthpoint-osd-adapter` is a bridge, not the project core.
The current adapter already exposes stable string-row views, and `healthpoint-rs` should not impose a tabular engine dependency on the core client just to support a future consumer.
Healthpoint-derived open publication and provider integration also remain governed by licensing and terms review.

## Decision

Keep the open_social_data bridge in its current string-row form and defer Arrow/Parquet conversion until there is a concrete consumer or a terms-approved provider path that justifies it.
Keep the actual `open_social_data` provider integration gated on terms review.

## Consequences

- The core Healthpoint crates stay free of tabular-engine coupling.
- The bridge remains easy to consume locally and easy to adapt later.
- Provider integration stays explicitly separated from the internal bridge decision.
