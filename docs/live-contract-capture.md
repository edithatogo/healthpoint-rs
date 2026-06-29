# Live contract capture plan

Live validation must use Dylan's licensed Healthpoint API key, but must not commit real Healthpoint payloads.

## Goals

Capture endpoint shape, not data:

- base URL;
- authentication scheme;
- search parameter names;
- pagination link/cursor shape;
- supported direct reads for `HealthcareService`, `Location`, and `Organization`;
- content type;
- response metadata headers;
- error body shape;
- rate-limit semantics if exposed.

## Redaction rules

Allowed to commit:

- request method;
- path template with IDs replaced, for example `/HealthcareService/{id}`;
- query parameter names, not user-sensitive values;
- status code;
- response header names and non-sensitive control values;
- FHIR `resourceType` names;
- field presence matrix such as `HealthcareService.telecom: present`.

Do not commit:

- API keys or bearer tokens;
- real IDs;
- real service names, addresses, phones, emails, URLs, eligibility text, comments, or raw FHIR bodies;
- screenshots of live API responses;
- cURL commands with credentials in shell history.

## Suggested capture matrix

Create a local untracked file such as `.healthpoint/live-contract-notes.toml`:

```toml
[auth]
base_url = "https://www.healthpointapi.com/"
scheme = "bearer" # bearer | x-api-key | header:<name>
validated_at = "2026-06-29"

[search.healthcare_service]
path = "/HealthcareService"
works = true
parameters = ["_count", "type", "category", "specialty"]
next_link = "absolute-same-origin"

[read.healthcare_service]
path_template = "/HealthcareService/{id}"
works = true

[read.location]
path_template = "/Location/{id}"
works = true

[read.organization]
path_template = "/Organization/{id}"
works = true

[headers]
content_type = "application/fhir+json"
has_etag = true
has_last_modified = false
has_retry_after = false
has_rate_limit_headers = false
```

After manual review, transpose only redacted endpoint-shape findings into `docs/api-assumptions.md` or a future `policy/healthpoint-access.toml` update.
