# Data export policy

Every generated data output should be accompanied by a manifest:

```json
{
  "schema_version": "healthpoint.export-manifest.v1",
  "contains_healthpoint_data": true,
  "access_policy": {
    "access_mode": "bring-your-own-key",
    "redistribution_status": "requires-review",
    "export_policy": "local-only",
    "public_cache_allowed": false
  }
}
```

## Allowed by default

- JSON output to stdout for an interactive user.
- Local manifest generation.
- Local JSONL/CSV/Parquet under `.healthpoint/` or user-selected paths.

## Requires explicit review

- Publishing data files to GitHub.
- Creating open dataset packs.
- Caching beyond a short local workflow.
- Running whole-directory bulk exports.

## Prohibited in this repo

- Real API responses as committed fixtures.
- API keys in examples, logs, tests, or docs.
- Public key-sharing proxy/server mode.
