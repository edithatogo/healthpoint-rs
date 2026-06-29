# ADR 0006: Use healthpoint:// resource URIs as adapter-facing identifiers

Status: accepted

## Context

CLI, MCP, and future catalogue integrations need a stable way to refer to Healthpoint resources without exposing Healthpoint API URLs or binding to a specific base URL.

## Decision

Define internal read-only resource URIs:

```text
healthpoint://service/{id}
healthpoint://location/{id}
healthpoint://organization/{id}
```

These are parsed and validated by `healthpoint-core` before any API request is built.

## Consequences

- MCP can expose a URI read tool before native resource templates are wired.
- Future open_social_data/catalogue integrations have stable identifiers.
- The URI scheme is not a claim about Healthpoint's upstream URL structure.
