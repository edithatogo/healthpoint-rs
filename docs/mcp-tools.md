# MCP surface

The MCP server is read-only and runs over stdio. It shares `healthpoint-client` with the CLI, so API-key handling, base URL configuration, provenance, and request limits remain consistent.

## Tools

```text
healthpoint.diagnostic.status
healthpoint.access.notes
healthpoint.access.policy
healthpoint.services.search
healthpoint.services.search_snomed
healthpoint.services.nearby
healthpoint.service.get
healthpoint.location.get
healthpoint.organization.get
healthpoint.resource.read
```

All tools are:

- read-only,
- synthetic by default and API-key gated only for live mode,
- result-limited,
- provenance-rich,
- designed to return JSON with source/access metadata,
- explicit about not being clinical decision support.

`healthpoint_read_resource_uri` accepts:

```text
healthpoint://service/{id}
healthpoint://organization/{id}
healthpoint://location/{id}
```

It mirrors the native MCP resource reader so clients can use either explicit tools or `resources/read`.

## Native resources and templates

Static resources:

```text
healthpoint://diagnostic/status
healthpoint://api/access-notes
healthpoint://access/policy
```

Resource templates:

```text
healthpoint://service/{id}
healthpoint://organization/{id}
healthpoint://location/{id}
healthpoint://query/services?text={text}&region={region}&limit={limit}
```

The dynamic templates are readable through `resources/read` and return JSON. The query template supports `text`, `region`, `branch-code`, `type`, `category`, `specialty`, and `limit` query parameters. Use `healthpoint.services.search` when a client prefers tool calls over resource reads.

## Prompts

```text
healthpoint_safe_search
healthpoint_license_check
```

These prompts keep Healthpoint usage read-only, attributed, local-only by default, and explicit about the no-public-cache/no-redistribution boundary.

## Guardrails

- No bulk dump tool without Healthpoint written approval.
- No public cache/proxy mode.
- No write tools.
- Secrets must never appear in tool outputs or MCP errors.
- Tool descriptions must not contain hidden behavioural instructions unrelated to the tool.
- Search tools should cap `_count` to 100 unless a reviewed bulk-export policy says otherwise.

## Glama tool-quality contract

Glama's visible score page was checked on 2026-06-30. It reported:

- Overall profile score: `83%`.
- Server Coherence: `A`, with `5/5` for disambiguation, naming consistency, tool count, and completeness.
- Tool Definition Quality: `A`, average `3.7/5` across `10` tools, lowest tool score `3.1/5`.
- Formula: overall quality is `70% Tool Definition Quality + 30% Server Coherence`; Tool Definition Quality is `60% mean TDQS + 40% minimum TDQS`.

For future tool changes, preserve the server-coherence strengths and optimize descriptions for the six Glama dimensions:

- Purpose Clarity: say what the tool does and how it differs from sibling tools.
- Usage Guidelines: say when to use it, when not to use it, and which sibling tool is the alternative.
- Behavioral Transparency: disclose read-only behavior, BYO-key/live-mode requirements, no-public-cache boundary, error behavior, result limits, pagination, and whether no live Healthpoint data is bundled.
- Parameter Semantics: explain non-obvious parameter formats, defaults, ranges, and filter interactions beyond the schema.
- Conciseness and Structure: keep the most important selection cue first and avoid hidden instructions.
- Contextual Completeness: mention return shape and provenance/access metadata when useful.

The main observed TDQS gaps were sparse usage guidance and sparse behavior/return-shape disclosure on search and get-by-id tools. Treat this section as the local contract for Glama scoring; `scripts/check-mcp-registry-submission.py` emits the same rubric under `glama_score_contract`.

## Launch examples

```bash
HEALTHPOINT_MODE=synthetic cargo run -p healthpoint-mcp
```

Claude-style local server entry:

```json
{
  "mcpServers": {
    "healthpoint": {
      "command": "healthpoint-mcp",
      "env": {
        "HEALTHPOINT_MODE": "live",
        "HEALTHPOINT_API_KEY": "...",
        "HEALTHPOINT_BASE_URL": "https://uat.healthpointapi.com/baseR4/",
        "HEALTHPOINT_AUTH_SCHEME": "x-api-key"
      }
    }
  }
}
```
