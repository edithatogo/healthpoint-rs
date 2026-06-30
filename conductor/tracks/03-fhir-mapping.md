# Track 03 — FHIR mapping

Status: partial

Implemented from synthetic fixtures:

- HealthcareService search bundle and single-resource mapping.
- Location mapping.
- Organization mapping.
- Bundle `next` and `total` extraction.
- Raw FHIR preservation.
- Provenance on every record.

Open:

- Validate against Healthpoint's actual FHIR profiles.
- ADR 0008 keeps tolerant JSON mapping until live profile validation justifies generated bindings.
