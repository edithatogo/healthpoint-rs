# MCP client configuration examples

## Local stdio server

```json
{
  "mcpServers": {
    "healthpoint": {
      "command": "healthpoint-mcp",
      "env": {
        "HEALTHPOINT_API_KEY": "...",
        "HEALTHPOINT_BASE_URL": "https://uat.healthpointapi.com/baseR4/",
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
        "HEALTHPOINT_API_KEY": "...",
        "HEALTHPOINT_BASE_URL": "https://uat.healthpointapi.com/baseR4/"
      }
    }
  }
}
```

Keep API keys in the MCP client's secret store or local config. Do not commit populated config files.
