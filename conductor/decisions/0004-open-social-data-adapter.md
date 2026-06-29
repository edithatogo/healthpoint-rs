# ADR 0004: `open_social_data` is a future adapter target, not the first core

## Status

Accepted.

## Context

`open_social_data` is built around open, reproducible social datasets and provider-backed tabular fetches. Healthpoint is a licensed FHIR directory API.

## Decision

Create `healthpoint-osd-adapter` as a thin tabular view contract. Do not add a hard dependency on `open_social_data` until the terms and view model are validated.

## Consequences

- Healthpoint remains FHIR/domain-first.
- Open-data outputs can be added without retrofitting the core.
- Licensing boundaries are clearer.
