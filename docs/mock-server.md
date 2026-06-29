# Synthetic mock Healthpoint server

`bin/mock-healthpoint-server` is a tiny stdlib-only Python server that serves the synthetic FHIR fixtures from `healthpoint-testkit` over HTTP.

It is designed for the gap between repository scaffolding and licensed live validation:

- no Healthpoint-origin payloads;
- no API key required;
- real HTTP status codes, headers, and paths;
- FHIR-ish `HealthcareService`, `Location`, `Organization`, and `CapabilityStatement` responses;
- same-origin pagination link for cursor validation.

## Run

```bash
bin/mock-healthpoint-server --port 8787
```

Then point the CLI at it after the Rust workspace compiles:

```bash
export HEALTHPOINT_BASE_URL="http://127.0.0.1:8787/"
export HEALTHPOINT_AUTH_SCHEME="none"

cargo run -p healthpoint-cli -- doctor
cargo run -p healthpoint-cli -- search services --snomed 171149006 --limit 1 --format json
cargo run -p healthpoint-cli -- get service svc-cervical-screening-1 --format json
cargo run -p healthpoint-cli -- get location loc-auckland-clinic-1 --format json
cargo run -p healthpoint-cli -- get organization org-example-provider-1 --format json
```

## Raw HTTP smoke test

```bash
curl -s 'http://127.0.0.1:8787/metadata' | jq .resourceType
curl -s 'http://127.0.0.1:8787/HealthcareService?_count=1&type=http%3A%2F%2Fsnomed.info%2Fsct%7C171149006' | jq .resourceType
```

## Boundaries

This mock server is not a compatibility guarantee for Healthpoint. It is a contract harness for the tool's assumptions. Real endpoint names, search parameters, authentication headers, pagination, and rate-limit behaviour must still be validated against licensed Healthpoint API documentation and Dylan's API key.
