# Track 06 — open_social_data bridge

Status: partial

Implemented views:

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

Implemented:

- Initial view data dictionaries in `docs/open-social-data-view-dictionaries.md`.

Open:

- Arrow/Parquet strategy is deferred; the string-row bridge remains the stable contract.
- Actual open_social_data provider only after terms review.
