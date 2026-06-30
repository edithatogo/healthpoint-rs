# FHIR mapping decision checkpoint

Date: 2026-06-30

## Completed internally

- Added ADR 0008 to codify the tolerant JSON FHIR mapping strategy.
- Confirmed the repository already preserves raw FHIR JSON and typed projections on top of it.
- Reduced the FHIR mapping track to the remaining live-profile validation gate.

## Remaining external gate

- Healthpoint live profile validation is still required before any future reconsideration of generated bindings.

## Result

- No repo-local work remains for the FHIR mapping strategy decision itself.
- The track now has an explicit decision artifact to reference in future live validation work.
