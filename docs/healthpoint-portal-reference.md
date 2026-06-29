# Healthpoint portal reference

Observed from the Healthpoint API portal on 2026-06-30. This file records non-secret implementation facts only.

## Portal state

- Account status: active.
- API key: stored only in local `.env` and GitHub Actions secret `HEALTHPOINT_API_KEY`.
- Terms accepted date shown by the portal: 2025-10-13.
- Terms page last updated: 7 May 2026.
- Portal build shown by `/environ.js`: `master-acf1767`, built 2026-05-27 NZST.
- Portal API Gateway config uses AWS API Gateway in `ap-southeast-2` with stage `prod`.

## Environments

| Environment | Base URL | Rate | Burst | Quota |
| --- | --- | ---: | ---: | ---: |
| UAT | `https://uat.healthpointapi.com/baseR4/` | 10 | 10 | 100000 |
| Production | production portal environment | 500 | 20 | 100000 |

Use UAT by default until Healthpoint gives written approval for production/live use.

## Authentication

All documented curl examples use:

```text
x-api-key: <key>
```

The client and CLI therefore default to:

```text
HEALTHPOINT_AUTH_SCHEME=x-api-key
```

## FHIR resources

Healthpoint documents the API as HL7 FHIR R4-aligned and exposes these main resources:

- `HealthcareService`
- `Location`
- `Practitioner`
- `PractitionerRole`

The documentation states that Healthpoint extends FHIR resources so that most user needs are met through `HealthcareService`, including linked location and practitioner information.

## Query parameters and value sets

Observed query/value-set documentation covers:

- `branch-code`
- `region`
- `dhb-region`
- `subregion`
- `services-provided-type`
- `specialty`
- `programme-region`
- `programme-area`
- `programme-type`
- `programme-age-groups`
- `programme-referral-type`
- `extra-details`
- `age-groups`
- `communication`
- `how-to-access`

The portal says query parameters should be URL-encoded.

## Recommended query patterns

Examples observed in the portal documentation:

```text
GET /HealthcareService?branch-code=primary&region=Southland
GET /HealthcareService?branch-code=primary&dhb-region=Southern
GET /HealthcareService?latitude=-44.9625917&longitude=168.0490921&radius=200
GET /HealthcareService?services-provided-type=COVID-19%20Vaccination
GET /HealthcareService?specialty=Orthopaedics
GET /HealthcareService?subregion=Ashburton&_format=xml
GET /HealthcareService/_search?_id=hp-service-32242&_format=json&_pretty=true
GET /Location/hp-location-31955
GET /PractitionerRole/hp-pracrole-36200
```

## Data samples observed

The portal lists JSON/XML samples for service families including allied health, cancer support, community health, dentistry, GP practice, mental health and addictions, midwifery, optometry, pharmacy, private/public services, social services, multi-branch services, locations, practitioner roles, and practitioners.

## Attribution and restrictions

Healthpoint attribution is required wherever information is used. Sandbox data must remain non-production and must not be redistributed, bulk-copied, used for AI training/dataset creation, or used to build a competing directory/data product without explicit written approval.
