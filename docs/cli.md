# CLI reference

The CLI is intentionally read-only. Commands either inspect configuration/requests, read the licensed Healthpoint API, or operate on synthetic fixtures.

## Offline commands

These never touch Healthpoint:

```bash
healthpoint doctor
healthpoint policy show
healthpoint fixture services --format json
healthpoint fixture location
healthpoint fixture organization
healthpoint schema service-record
healthpoint inspect search-url --text "cervical screening" --snomed 171149006
healthpoint inspect resource-url HealthcareService svc-example
```

`inspect search-url` validates query arguments and renders the URL that would be requested. It does not send the request and does not require an API key.

## Live read commands

These require a valid API key/licence:

```bash
healthpoint search services --text "cervical screening" --limit 10 --format json
healthpoint search services --snomed 171149006 --limit 10 --format human
healthpoint search services --lat -36.8485 --lon 174.7633 --radius-km 10 --format csv
healthpoint get service <id> --format json
healthpoint get location <id> --format json
healthpoint get organization <id> --format json
healthpoint get uri healthpoint://service/<id> --format json
```

## Local export commands

Exports remain local by default and carry a manifest sidecar:

```bash
healthpoint export services \
  --text "cervical screening" \
  --limit 25 \
  --format jsonl \
  --output .healthpoint/cervical-screening.jsonl
```

Default manifest path:

```text
<output>.manifest.json
```

The manifest is intentionally blunt: it says the output contains Healthpoint-derived data and must not be redistributed unless the user's Healthpoint terms permit it.
