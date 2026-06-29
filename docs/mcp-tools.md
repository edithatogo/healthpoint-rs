# MCP surface

The MCP server is read-only and runs over stdio. It shares `healthpoint-client` with the CLI, so API-key handling, base URL configuration, provenance, and request limits remain consistent.

## Tools in the current spike

```text
healthpoint_diagnostic_status
healthpoint_api_access_notes
healthpoint_access_policy
healthpoint_search_services
healthpoint_search_by_snomed
healthpoint_find_nearby_services
healthpoint_get_service
healthpoint_get_location
healthpoint_get_organization
healthpoint_read_resource_uri
```

All tools are:

- read-only,
- API-key gated via environment configuration,
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

It mirrors planned MCP resources while avoiding compile-risk from resource-template wiring until the selected RMCP API is validated with a local Rust toolchain.

## Planned resource templates

```text
healthpoint://service/{id}
healthpoint://organization/{id}
healthpoint://location/{id}
healthpoint://query/services?type={code}&limit={limit}
```

Resource support is deliberately deferred until the RMCP API is validated locally, because the current dependency is intentionally tracking `modelcontextprotocol/rust-sdk` main.

## Guardrails

- No bulk dump tool until licensing/rate-limit terms are reviewed.
- No public cache/proxy mode.
- No write tools.
- Secrets must never appear in tool outputs or MCP errors.
- Tool descriptions must not contain hidden behavioural instructions unrelated to the tool.
- Search tools should cap `_count` to 100 unless a reviewed bulk-export policy says otherwise.

## Launch examples

```bash
HEALTHPOINT_API_KEY=... cargo run -p healthpoint-mcp
```

Claude-style local server entry:

```json
{
  "mcpServers": {
    "healthpoint": {
      "command": "healthpoint-mcp",
      "env": {
        "HEALTHPOINT_API_KEY": "...",
        "HEALTHPOINT_BASE_URL": "https://uat.healthpointapi.com/baseR4/",
        "HEALTHPOINT_AUTH_SCHEME": "x-api-key"
      }
    }
  }
}
```
