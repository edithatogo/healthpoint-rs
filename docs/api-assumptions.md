# API assumptions to validate

The scaffold assumes a FHIR-style REST API with resources such as:

```text
GET /HealthcareService
GET /HealthcareService/{id}
GET /Organization/{id}
GET /Location/{id}
```

The public Healthpoint page confirms HL7 FHIR and SNOMED CT orientation, but does not expose full developer documentation.

## Configurable assumptions

- `HEALTHPOINT_BASE_URL` sets the API root.
- `HEALTHPOINT_AUTH_SCHEME` sets bearer/header/none auth.
- `HEALTHPOINT_GEO_SEARCH_MODE` sets nearby encoding: `healthpoint-lat-lon` or `fhir-near`.
- `HEALTHPOINT_TIMEOUT_SECS` sets the per-request timeout, clamped to 1..300 seconds.
- Text search currently encodes as `_content=<term>`.
- Service type encodes as `type=<system|code>`.
- Category encodes as `category=<system|code>`.
- Specialty encodes as `specialty=<system|code>`.
- Nearby search defaults to custom `latitude`, `longitude`, `radius_km` parameters.
- FHIR-next pagination links are accepted only when they have the same origin as `HEALTHPOINT_BASE_URL`.

## Validation tasks

- Confirm production and UAT base URLs.
- Confirm auth header name and token format.
- Confirm supported FHIR search parameters.
- Confirm paging format and next-link/cursor behaviour.
- Confirm geospatial parameter names and units.
- Confirm whether `Location` and `Organization` are directly readable.
- Confirm whether `_content` or another Healthpoint-specific text search parameter is preferred.

## Current code posture

The client is intentionally permissive in configuration and conservative in behaviour. It does not assume any unpublished endpoint contract is guaranteed. Validate against licensed API documentation before cutting a release.
