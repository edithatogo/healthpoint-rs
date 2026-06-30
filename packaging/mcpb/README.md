# Smithery MCPB packaging

Smithery's local-server path distributes stdio MCP servers as MCPB bundles. This
directory contains the source metadata used by `scripts/package-mcpb`.

The generated bundle is written to `target/mcpb/healthpoint-rs.mcpb` and is not
committed. It contains:

- `manifest.json`
- the `healthpoint-mcp` binary for the current platform
- `README.md`
- `LICENSE`
- `server.json`

No Healthpoint API key, `.env` file, cached payload, export, trace, or real API
response is included.

Build a local bundle with:

```bash
scripts/package-mcpb
```

The generated manifest declares `HEALTHPOINT_API_KEY` as a required secret
configuration value supplied by the installing user.

## Metadata source

`manifest-metadata.json` is the source of truth for Smithery-facing MCPB metadata. It intentionally mirrors the actual MCP server surface:

- 10 tools
- 3 static resources
- 4 resource templates
- 2 prompts

After editing metadata, run:

```bash
scripts/package-mcpb
scripts/check-smithery-compliance.py
```
