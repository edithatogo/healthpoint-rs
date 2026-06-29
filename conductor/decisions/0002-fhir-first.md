# ADR 0002: Preserve FHIR as the canonical internal representation

## Status

Accepted.

## Context

Healthpoint publicly describes the API as HL7 FHIR and SNOMED CT based. FHIR resources are graph-shaped, not naturally tabular.

## Decision

Store typed domain records with raw FHIR JSON preserved. Generate tables as views, not the other way around.

## Consequences

- CLI/MCP can expose rich resource structure.
- Future open_social_data integration receives stable, documented tabular views.
- The mapper can remain tolerant while endpoint details are validated.
