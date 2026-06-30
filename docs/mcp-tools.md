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
