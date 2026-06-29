# Data export policy

Every generated data output should be accompanied by a manifest:

```json
{
  "schema_version": "healthpoint.export-manifest.v1",
  "contains_healthpoint_data": true,
  "access_policy": {
    "access_mode": "bring-your-own-key",
    "redistribution_status": "prohibited-without-written-approval",
    "export_policy": "local-only",
    "public_cache_allowed": false
  }
}
```

## Allowed by default

- JSON output to stdout for an interactive user.
- Local manifest generation.
- Local JSONL/CSV/Parquet under `.healthpoint/` or user-selected paths.
- Academic research analysis outputs within Dylan/VUW's executed permitted purpose, provided publication attribution is preserved.

## Requires explicit review

- Publishing data files to GitHub.
- Creating open dataset packs.
- Caching beyond a short local workflow.
- Running whole-directory bulk exports.
- Any production workflow outside the executed academic non-commercial research scope.

## Prohibited in this repo

- Real API responses as committed fixtures.
- API keys in examples, logs, tests, or docs.
- Public key-sharing proxy/server mode.
- AI training, model development, benchmark datasets, or derived datasets using Healthpoint licensed material.
- Competing provider directories, public mirrors, or public open-data packs without Healthpoint written approval.
