# MCP Registry OCI submission closeout

Date: 2026-06-30

## Completed

- Authenticated `mcp-publisher` through GitHub device flow.
- Confirmed the official MCP Registry live publisher rejects `registryType: cargo`.
- Added a runtime OCI image path for `healthpoint-mcp` with the required `io.modelcontextprotocol.server.name` label.
- Published `ghcr.io/edithatogo/healthpoint-mcp:0.1.0` and `latest`.
- Updated `server.json` to use the official registry-supported OCI identifier format: `ghcr.io/edithatogo/healthpoint-mcp:0.1.0`.
- Updated the MCP registry validator and docs to encode the live OCI validation rules.

## Evidence

- `scripts/check-mcp-registry-submission.py` passes with no errors.
- `docker push ghcr.io/edithatogo/healthpoint-mcp:0.1.0` succeeded with digest `sha256:ba9fe0c4e37dd356d53e1a9fca1cd75067e80b3bb8bc26e9c11501731d56310c`.
- `mcp-publisher publish server.json` now reaches registry package validation for the OCI image.

## Remaining blocker

The official MCP Registry rejects the GHCR image because the package is private:

```text
OCI image 'ghcr.io/edithatogo/healthpoint-mcp:0.1.0' is private or requires authentication. Only public images are supported
```

GitHub REST package visibility update attempts returned `404` for both nested and top-level package names. The remaining action is to make `healthpoint-mcp` public in the GitHub package settings UI, then rerun:

```bash
~/.local/bin/mcp-publisher publish server.json
```
