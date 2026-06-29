# MCP packaging

The MCP server is packaged as the `healthpoint-mcp` binary.

## Local install

```bash
cargo install --path crates/healthpoint-mcp
```

The server uses stdio transport and reads configuration from environment variables declared in `server.json`.

## Required runtime configuration

- `HEALTHPOINT_API_KEY`: user-provided licensed Healthpoint API key or token.
- `HEALTHPOINT_BASE_URL`: optional API base URL; defaults to `https://uat.healthpointapi.com/baseR4/`.
- `HEALTHPOINT_AUTH_SCHEME`: optional auth mode; defaults to `x-api-key`.
- `HEALTHPOINT_GEO_SEARCH_MODE`: optional nearby-search encoding mode.
- `HEALTHPOINT_TIMEOUT_SECS`: optional HTTP timeout.

Do not package API keys, `.env` files, cached Healthpoint payloads, generated exports, logs, traces, or real API responses.

## Release artifact layout

Release archives include:

- `healthpoint-mcp` or `healthpoint-mcp.exe`
- `healthpoint` or `healthpoint.exe`
- `server.json`
- `README.md`
- `LICENSE`

Consumers should place the binary on `PATH` or configure their MCP client with an absolute command path.

## Client configuration

Example client snippets live in `docs/integrations/mcp-client-configs.md`.

Before registry submission, validate the installed binary with a real MCP client against synthetic or redacted metadata-only flows.
