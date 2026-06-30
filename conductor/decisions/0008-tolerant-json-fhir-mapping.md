# ADR 0008: Keep tolerant JSON FHIR mapping until live profiles justify generated bindings

## Status

Accepted.

## Context

`healthpoint-rs` already preserves raw FHIR JSON on every typed record and uses typed projections for CLI, MCP, and export use.
The Healthpoint live profile, extension, and common payload shape are not yet fully validated, so a generated-binding layer would add rigidity before the contract is stable.

## Decision

Keep tolerant JSON extraction for `HealthcareService`, `Location`, and `Organization` mapping.
Revisit generated FHIR bindings only after live profile validation demonstrates a stable, high-value shape that justifies the added maintenance burden.

## Consequences

- The mapper stays resilient to Healthpoint-specific extensions and fields.
- CLI, MCP, and export surfaces keep working against the current synthetic and live-documented shapes.
- Generated bindings remain a future option, not a current requirement.
