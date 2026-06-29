# MCP surface

## Initial tools

```text
healthpoint_search_services
healthpoint_get_service
healthpoint_get_organization
```

All tools are:

- read-only,
- API-key gated via environment configuration,
- result-limited,
- provenance-rich,
- designed to return JSON with source/access metadata.

## Planned resource templates

```text
healthpoint://service/{id}
healthpoint://organization/{id}
healthpoint://location/{id}
healthpoint://query/services?type={code}&limit={limit}
```

## Guardrails

- No bulk dump tool until licensing/rate-limit terms are reviewed.
- No public cache/proxy mode.
- No write tools.
- Secrets must never appear in tool outputs or MCP errors.
- Tool descriptions must not contain hidden behavioural instructions unrelated to the tool.
