# MCP packaging

The MCP server is packaged as the `healthpoint-mcp` binary.

## Local install

```bash
cargo install --path crates/healthpoint-mcp
```

The server uses stdio transport and reads configuration from environment variables declared in `server.json`.

## OCI image

The official MCP Registry currently validates this server through the GHCR OCI image declared in `server.json`:

```bash
docker run --rm -i \
  -e HEALTHPOINT_API_KEY \
  ghcr.io/edithatogo/healthpoint-mcp:0.1.0
```

The image sets the required MCP ownership label:

```text
io.modelcontextprotocol.server.name=io.github.edithatogo/healthpoint-rs
```

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

## Smithery local bundle

Smithery's local-server publishing path uses MCPB bundles for stdio servers. Generate a platform-specific bundle with:

```bash
scripts/package-mcpb
```

The bundle is written under `target/mcpb/` and is intentionally ignored by git. It includes the `healthpoint-mcp` binary, `manifest.json`, `server.json`, `README.md`, and `LICENSE`. The generated manifest declares `HEALTHPOINT_API_KEY` as required sensitive user configuration and does not include any secret value.

Submit the generated `.mcpb` through Smithery's local MCPB publishing flow:

```bash
smithery mcp publish target/mcpb/healthpoint-rs-<version>-<os>-<arch>.mcpb -n healthpoint-rs
```

If Smithery requires scoped naming, use the account or organization scope shown by the logged-in Smithery UI.

The 0.1.0 Darwin/arm64 bundle was accepted by Smithery on 2026-06-30:

```text
qualifiedName: edithatogo/healthpoint-rs
deploymentId: c73eb36e-66ba-4d28-b95e-71b92dcf20f2
statusUrl: https://smithery.ai/servers/edithatogo/healthpoint-rs/releases
mcpUrl: https://healthpoint-rs--edithatogo.run.tools
```

After adding the Smithery README badge, a follow-up 0.1.0 Darwin/arm64 release
was accepted as deployment `f68c78e1-5eec-4c4b-a530-c34291c84819`.

## Client configuration

Example client snippets live in `docs/integrations/mcp-client-configs.md`.

Before registry submission, validate the installed binary with a real MCP client against synthetic or redacted metadata-only flows.
