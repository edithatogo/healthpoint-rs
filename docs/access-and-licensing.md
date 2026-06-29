# Access and licensing plan

## Operating model

`healthpoint-rs` is open-source code for licensed users of the Healthpoint API.

The default mode is:

- user supplies their own API key,
- queries are read-only,
- no real Healthpoint payloads are committed,
- local caches/exports are ignored by Git,
- generated exports include a manifest warning that redistribution is not assumed.

## Modes

### Tool-only mode

The safest default. The tool queries Healthpoint and prints/returns results to the licensed user. No durable export is produced unless explicitly requested.

### Licensed local data mode

The user creates local JSON/CSV/Parquet files under their own licence/access terms. Each output receives a manifest.

### Open-approved mode

Disabled by default. Only use when Healthpoint terms or a specific written permission allow publication/redistribution of the relevant data/derived view.

## Machine-readable policy stub

The conservative default policy lives at `policy/healthpoint-access.toml`. It should remain `requires-review` until API terms/licence details are reviewed.

## Questions to answer from Healthpoint terms/API documentation

- Authentication header/token requirements.
- Allowed caching duration and storage constraints.
- Bulk export and rate-limit constraints.
- Attribution requirements.
- Whether derived aggregate or tabular outputs may be redistributed.
- Whether public mirrors/proxies are prohibited.
- Required currency/disclaimer language.

## Non-goals

- Scraping Healthpoint websites.
- Avoiding API access controls.
- Sharing a single API key through a public proxy.
- Publishing a Healthpoint data mirror without explicit permission.
