# MCP client configuration examples

## Local stdio server

```json
{
  "mcpServers": {
    "healthpoint": {
      "command": "healthpoint-mcp",
      "env": {
        "HEALTHPOINT_API_KEY": "...",
        "HEALTHPOINT_BASE_URL": "https://www.healthpointapi.com/",
        "HEALTHPOINT_AUTH_SCHEME": "bearer",
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
        "HEALTHPOINT_API_KEY": "...",
        "HEALTHPOINT_BASE_URL": "https://www.healthpointapi.com/"
      }
    }
  }
}
```

Keep API keys in the MCP client's secret store or local config. Do not commit populated config files.
