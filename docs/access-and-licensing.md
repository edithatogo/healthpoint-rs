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

## Machine-readable policy

The conservative default policy lives at `policy/healthpoint-access.toml`. It now reflects the portal review and the June 2026 licence variation retrieved from VUW Outlook. The contract evidence is stored only under ignored local storage and must not be committed.

## Questions to answer from Healthpoint terms/API documentation

- Authentication header/token requirements.
- Allowed caching duration and storage constraints.
- Exact publication acknowledgement wording for each output venue.
- Whether any aggregate, tabular, or open_social_data-derived output can be redistributed beyond academic publications.
- Whether any public demo, hosted MCP server, or proxy would require a separate written approval.
- Required currency/disclaimer language for published research artifacts.

## Non-goals

- Scraping Healthpoint websites.
- Avoiding API access controls.
- Sharing a single API key through a public proxy.
- Publishing a Healthpoint data mirror without explicit permission.
