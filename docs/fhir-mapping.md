# FHIR mapping notes

`healthpoint-rs` keeps raw FHIR JSON on every typed record. The typed model is a projection for CLI, MCP, and export use, not a replacement for the source resource.

## HealthcareService projection

Current `ServiceRecord` fields:

```text
id
identifiers
name
active
provided_by
locations
coverage_areas
endpoints
categories
service_types
specialties
service_provision_codes
programs
characteristics
communications
referral_methods
eligibilities
appointment_required
comment
extra_details
available_times
not_available
contacts
provenance
raw_fhir
```

FHIR fields not yet projected remain available through `raw_fhir`.

## Location projection

Current `LocationRecord` fields:

```text
id
identifiers
name
status
mode
location_types
physical_types
contacts
address
position
managing_organization
part_of
endpoints
hours_of_operation
provenance
raw_fhir
```

## Organization projection

Current `OrganizationRecord` fields:

```text
id
identifiers
organization_types
name
aliases
active
part_of
endpoints
contacts
provenance
raw_fhir
```

## Why not generated full FHIR bindings yet?

FHIR code generation can be heavy and brittle for a first API-client spike. The current mapper uses tolerant JSON extraction so the client can validate Healthpoint's actual profile quickly. Revisit generated bindings after the live endpoint contract, extensions, profiles, and common payload shapes are known.
