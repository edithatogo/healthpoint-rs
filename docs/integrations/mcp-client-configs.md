# MCP client configuration examples

## Local stdio server

```json
{
  "mcpServers": {
    "healthpoint": {
      "command": "healthpoint-mcp",
      "env": {
        "HEALTHPOINT_MODE": "live",
        "HEALTHPOINT_MODE": "synthetic",
        "HEALTHPOINT_AUTH_SCHEME": "x-api-key",
        "HEALTHPOINT_GEO_SEARCH_MODE": "healthpoint-lat-lon"
      }
    }
  }
}
```

## Source checkout during development

```json
{
  "mcpServers": {
    "healthpoint-dev": {
      "command": "cargo",
      "args": ["run", "-p", "healthpoint-mcp"],
      "env": {
        "HEALTHPOINT_MODE": "synthetic"
      }
    }
  }
}
```

Keep API keys in the MCP client's secret store or local config. Do not commit populated config files.

Synthetic mode is the default and requires no credentials. Set `HEALTHPOINT_MODE=live` and provide `HEALTHPOINT_API_KEY` only for licensed live Healthpoint API calls.
