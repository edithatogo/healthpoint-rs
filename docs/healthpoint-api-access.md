# Healthpoint API access notes

These notes record facts observed from the Healthpoint API portal on 2026-06-30.
Do not commit API keys, portal screenshots, raw API responses, HAR files, traces, or real Healthpoint payloads.

## Authentication

The portal exposes a single API key for the account. The key is stored locally in `.env`, which is ignored by git.

Use the key as an `x-api-key` header:

```bash
curl -H "x-api-key:$HEALTHPOINT_API_KEY" \
  "https://uat.healthpointapi.com/baseR4/HealthcareService?branch-code=primary&region=Southland&_count=1"
```

The repository should use:

```bash
HEALTHPOINT_BASE_URL=https://uat.healthpointapi.com/baseR4/
HEALTHPOINT_AUTH_SCHEME=x-api-key
```

A tiny UAT probe against `HealthcareService` returned HTTP 200 with a FHIR `Bundle` and one entry when `_count=1` was supplied.

## Environments and limits

The portal listed two environments:

| Environment | Description | Rate | Burst | Quota |
| --- | --- | ---: | ---: | ---: |
| Production | prod usage plan | 500 | 20 | 100000 |
| UAT | uat usage plan | 10 | 10 | 100000 |

The dashboard usage window observed was 2026-05-30 to 2026-07-01.

The June 2026 API and Data Licence variation retrieved from VUW Outlook specifies production limits for the executed academic non-commercial research use: 100,000 API calls per day and 350 requests per second. Use the lower portal-observed or contract-stated limit when in doubt, and keep live validation metadata-only.

## Data model

Healthpoint documents the API as HL7 FHIR R4-aligned. Primary resources observed in the documentation:

- `HealthcareService`
- `Location`
- `Practitioner`
- `PractitionerRole`

The documentation states that most use cases are satisfied by `HealthcareService` queries, especially combinations of branch and region. It also states that Healthpoint extends FHIR resources to include Healthpoint-specific information and links between services, locations, practitioners, and practitioner roles.

## Query patterns observed

Examples in the documentation use GET requests against UAT paths such as:

```text
https://uat.healthpointapi.com/baseR4/HealthcareService?branch-code=primary&region=Southland
https://uat.healthpointapi.com/baseR4/HealthcareService?branch-code=primary&dhb-region=Southern
https://uat.healthpointapi.com/baseR4/HealthcareService?latitude=-44.9625917&longitude=168.0490921&radius=200
https://uat.healthpointapi.com/baseR4/HealthcareService?services-provided-type=COVID-19%20Vaccination
https://uat.healthpointapi.com/baseR4/HealthcareService?specialty=Orthopaedics
https://uat.healthpointapi.com/baseR4/HealthcareService?subregion=Ashburton&_format=xml
https://uat.healthpointapi.com/baseR4/HealthcareService/_search?_id=hp-service-32242&_format=json&_pretty=true
https://uat.healthpointapi.com/baseR4/Location/hp-location-31955
https://uat.healthpointapi.com/baseR4/PractitionerRole/hp-pracrole-36200
```

The documentation says query parameters should be URL-encoded.

## Attribution

The documentation requires attribution to Healthpoint as the source wherever the information is used. The observed suggested wording begins with:

```text
This specialist profile information has been provided by https://www.healthpoint.co.nz, helping people better understand and use New Zealand health services
```

Use this requirement as a product and export gate.
