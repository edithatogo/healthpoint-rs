# open_social_data adapter plan

`healthpoint-osd-adapter` is a bridge, not the project core. Healthpoint remains FHIR/domain-first; open_social_data can later consume approved tabular views.

## Current views

```text
services
locations
organizations
service-locations
service-codes
service-contacts
service-eligibilities
service-availability
```

The adapter currently returns simple string rows so it can later be converted to Polars, Arrow, or Parquet without forcing the core Healthpoint client to depend on a tabular engine.

## Licensing gates

Do not publish Healthpoint-derived data into open dataset packs until terms or written permission explicitly allow the relevant output. The adapter may be used locally by licensed users before any open publication path exists.

## Future provider shape

A future open_social_data integration should map roughly to:

```text
provider: healthpoint
access_mode: bring-your-own-key
redistribution_status: requires-review | allowed
views:
  - services
  - locations
  - organizations
  - service_locations
  - service_codes
  - service_contacts
  - service_eligibilities
  - service_availability
```

Each generated dataset pack should carry:

- source URL/base URL,
- retrieval timestamp,
- Healthpoint attribution/disclaimer if required,
- access mode,
- redistribution status,
- quality report path,
- schema/data dictionary,
- refresh policy.

## View dictionaries

Column dictionaries live in `docs/open-social-data-view-dictionaries.md`.

The CLI exposes the same stable dictionaries:

```bash
healthpoint osd views --format json
```

## Adapter contract

The adapter must never infer that Healthpoint-derived rows are open. It should pass through `AccessPolicy` and `SourceProvenance` from `healthpoint-core` into any future catalogue or quality-report layer.
