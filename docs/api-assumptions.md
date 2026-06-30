# API assumptions and live findings

This file records Healthpoint API facts already observed from the portal and the remaining shape questions that still need a live key-backed probe.
Do not commit API keys, portal screenshots, raw API responses, HAR files, traces, or real Healthpoint payloads.

## Observed live facts

- `HEALTHPOINT_BASE_URL` resolves to `https://uat.healthpointapi.com/baseR4/` in the documented UAT configuration.
- `HEALTHPOINT_AUTH_SCHEME` is `x-api-key`.
- A tiny UAT probe against `HealthcareService` returned HTTP 200 with a FHIR `Bundle` and one entry when `_count=1` was supplied.

## Documented query shapes

The scaffold assumes a FHIR-style REST API with resources such as:

```text
GET /HealthcareService
GET /HealthcareService/{id}
GET /Organization/{id}
GET /Location/{id}
```

The public Healthpoint page confirms HL7 FHIR and SNOMED CT orientation, but does not expose full developer documentation.

Observed and documented search/query parameters include:

- `branch-code`
- `region`
- `dhb-region`
- `latitude`
- `longitude`
- `radius`
- `services-provided-type`
- `specialty`
- `subregion`
- `_count`
- `_format`
- `_pretty`
- `_id`

The current code/config posture still allows these client assumptions to vary:

- `HEALTHPOINT_GEO_SEARCH_MODE` sets nearby encoding: `healthpoint-lat-lon` or `fhir-near`.
- `HEALTHPOINT_TIMEOUT_SECS` sets the per-request timeout, clamped to 1..300 seconds.
- Text search currently encodes as `_content=<term>`.
- Service type encodes as `type=<system|code>`.
- Category encodes as `category=<system|code>`.
- Specialty encodes as `specialty=<system|code>`.
- Nearby search defaults to custom `latitude`, `longitude`, `radius_km` parameters.
- FHIR-next pagination links are accepted only when they have the same origin as `HEALTHPOINT_BASE_URL`.

## Remaining live-only confirmations

- Confirm paging format and next-link/cursor behaviour.
- Confirm whether `Location` and `Organization` are directly readable in live UAT.
- Confirm error/status/rate-limit response headers.
- Confirm whether `_content` or another Healthpoint-specific text search parameter is preferred.

## Current code posture

The client is intentionally permissive in configuration and conservative in behaviour. It does not assume any unpublished endpoint contract is guaranteed. Validate against licensed API documentation before cutting a release.
