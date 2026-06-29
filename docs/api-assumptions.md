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
- Text search currently encodes as `_content=<term>`.
- Service type encodes as `type=<system|code>`.
- Category encodes as `category=<system|code>`.
- Specialty encodes as `specialty=<system|code>`.
- Nearby search currently defaults to custom `latitude`, `longitude`, `radius_km` parameters.

## Validation tasks

- Confirm production and UAT base URLs.
- Confirm auth header name and token format.
- Confirm supported FHIR search parameters.
- Confirm paging format and next-link/cursor behaviour.
- Confirm geospatial parameter names and units.
- Confirm whether `Location` and `Organization` are directly readable.
