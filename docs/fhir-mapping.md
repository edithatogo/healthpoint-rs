# FHIR mapping notes

`healthpoint-rs` keeps raw FHIR JSON on every typed record. The typed model is a projection for CLI, MCP, and export use, not a replacement for the source resource.

## HealthcareService projection

Current `ServiceRecord` fields:

```text
id
name
active
provided_by
locations
coverage_areas
categories
service_types
specialties
programs
communications
referral_methods
appointment_required
contacts
provenance
raw_fhir
```

FHIR fields not yet projected remain available through `raw_fhir`.

## Location projection

Current `LocationRecord` fields:

```text
id
name
status
mode
location_types
contacts
address
position
managing_organization
provenance
raw_fhir
```

## Organization projection

Current `OrganizationRecord` fields:

```text
id
name
active
contacts
provenance
raw_fhir
```

## Why not generated full FHIR bindings yet?

FHIR code generation can be heavy and brittle for a first API-client spike. The current mapper uses tolerant JSON extraction so the client can validate Healthpoint's actual profile quickly. Revisit generated bindings after the live endpoint contract, extensions, profiles, and common payload shapes are known.
