# Official MCP Registry published

Date: 2026-06-30

## Completed

- Made the `healthpoint-mcp` GHCR package public through GitHub package settings.
- Verified package visibility via GitHub API: `healthpoint-mcp` is `public`.
- Refreshed `mcp-publisher` GitHub authentication through device authorization.
- Published `server.json` to the official MCP Registry.

## Evidence

```text
Publishing to https://registry.modelcontextprotocol.io...
✓ Successfully published
✓ Server io.github.edithatogo/healthpoint-rs version 0.1.0
```

The local MCP registry validator also passes with no errors:

```bash
scripts/check-mcp-registry-submission.py
```

## Remaining gates

- Third-party MCP directories remain account-gated or directory-indexing gated.
- Healthpoint written approval is still required before production use, hosted proxying, public data redistribution, AI dataset use, or non-research commercial use.
