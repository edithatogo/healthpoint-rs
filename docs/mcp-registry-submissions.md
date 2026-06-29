# MCP registry submissions

`server.json` is the canonical local manifest. `scripts/check-mcp-registry-submission.py` validates the local submission prerequisites and emits the registry matrix used by `.github/workflows/mcp-registry-submission.yml`.

## Registry matrix

| Registry | URL | Submission route | Current automation status |
| --- | --- | --- | --- |
| Official MCP Registry | <https://github.com/modelcontextprotocol/registry> | `mcp-publisher login && mcp-publisher publish` after crates.io package publication | Automated readiness check; publish step gated on `mcp-publisher` credentials/tooling |
| Smithery | <https://smithery.ai/> | Provider/account submission or repository import | Manual account-gated submission |
| Glama MCP server directory | <https://glama.ai/mcp/servers> | Directory submission/indexing | Manual account-gated submission |
| PulseMCP | <https://www.pulsemcp.com/> | Directory submission/indexing | Manual account-gated submission |
| mcp.so | <https://mcp.so/> | Directory submission/indexing | Manual account-gated submission |

## Submission requirements

- Public source repository with Apache-2.0 licence.
- Published crates.io packages for `healthpoint-cli` and `healthpoint-mcp` at the release version.
- `server.json` version matches `workspace.package.version`.
- `server.json` marks `HEALTHPOINT_API_KEY` as a secret.
- MCP server is read-only and BYO-key.
- No Healthpoint API key, live payload, export, trace, cache, screenshot, or licensed dataset is bundled.
- Healthpoint redistribution remains local-only unless written approval is obtained.

## Verification loop

1. Run `scripts/check-release-version.py --tag vX.Y.Z`.
2. Run `scripts/check-mcp-registry-submission.py`.
3. Publish crates in dependency order.
4. Re-run `.github/workflows/mcp-registry-submission.yml`.
5. Submit to registries with account-gated forms or publisher tooling.
6. Record accepted/rejected/pending status in a GitHub issue without pasting credentials or confidential contract text.
