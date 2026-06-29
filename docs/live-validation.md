# Live validation checklist

Live Healthpoint checks are deliberately key-gated and should not run in CI by default.

## Before running

```bash
cp .env.example .env
$EDITOR .env
cargo run -p healthpoint-cli -- doctor
```

Use the verified UAT defaults unless Healthpoint has explicitly issued different credentials:

```bash
HEALTHPOINT_BASE_URL=https://uat.healthpointapi.com/baseR4/
HEALTHPOINT_AUTH_SCHEME=x-api-key
HEALTHPOINT_EXPORT_POLICY=local-only
```

Confirm that `doctor` reports:

```json
{
  "api_key_present": true
}
```

## Offline preflight

```bash
cargo run -p healthpoint-cli -- fixture services --format json
cargo run -p healthpoint-cli -- schema service-record
cargo run -p healthpoint-cli -- inspect search-url --text "cervical screening" --limit 5
cargo run -p healthpoint-cli -- inspect resource-url HealthcareService synthetic-id
```

## Smoke tests

```bash
cargo run -p healthpoint-cli -- search services --text "cervical screening" --limit 5 --format json
cargo run -p healthpoint-cli -- search services --snomed 171149006 --limit 5 --format json
cargo run -p healthpoint-cli -- search services --lat -36.8485 --lon 174.7633 --radius-km 10 --limit 5 --format json
```

Then copy a returned id/reference and test:

```bash
cargo run -p healthpoint-cli -- get service <service-id> --format json
cargo run -p healthpoint-cli -- get location <location-id> --format json
cargo run -p healthpoint-cli -- get organization <organization-id> --format json
cargo run -p healthpoint-cli -- get uri healthpoint://service/<service-id> --format json
```

## What to record

Record only metadata, never real API payloads:

- base URL shape,
- auth scheme/header name,
- working search params,
- observed paging shape,
- whether Location and Organization reads work,
- status/error shapes,
- rate-limit headers if supplied,
- any required attribution/disclaimer language.

Write findings to `docs/api-assumptions.md` and update the relevant Conductor track.

See also:

- `docs/healthpoint-api-access.md`
- `docs/healthpoint-license-notes.md`

## GitHub Actions smoke

The repository has a manual `Live smoke` workflow. It uses the `HEALTHPOINT_API_KEY` repository secret and runs `healthpoint smoke`, which confirms authentication and parsing without printing Healthpoint payloads.
