# MCP registry submissions

This document records the current submission requirements checked on 2026-06-30 and the project-specific status for `healthpoint-rs`. The canonical local validator is `scripts/check-mcp-registry-submission.py`; the GitHub workflow is `.github/workflows/mcp-registry-submission.yml`.

## Project submission identity

- Official MCP Registry server name: `io.github.edithatogo/healthpoint-rs`.
- OCI package to register: `ghcr.io/edithatogo/healthpoint-rs/healthpoint-mcp:0.1.0`.
- Rust package distribution path: `healthpoint-mcp` on crates.io.
- Transport: `stdio`.
- Distribution path: GHCR OCI image for the official MCP Registry, crates.io for Rust users, and GitHub Release binaries as additional convenience distributions.
- Secret configuration: `HEALTHPOINT_API_KEY` is required and marked secret.
- Safety boundary: BYO-key, read-only tools/resources/prompts, no bundled Healthpoint data, no public redistribution of licensed Healthpoint payloads.

## Registry matrix

| Registry | Current requirements checked | healthpoint-rs status | Submission gate |
| --- | --- | --- | --- |
| Official MCP Registry | Uses `server.json`; registry only stores metadata; package must be published first; publisher CLI is `mcp-publisher`; GitHub auth requires an `io.github.<user>/...` server name. Live publisher validation currently rejects `registryType: cargo` and accepts supported package ownership mechanisms such as OCI. OCI images require the `io.modelcontextprotocol.server.name` image label, and OCI package versions must be encoded in `identifier` rather than a separate `version` field. Sources: official quickstart/package-types docs and live `mcp-publisher publish` validation on 2026-06-30. | Manifest now uses GHCR OCI package metadata for registry submission. Crates.io remains published for Rust users. | Requires publishing `ghcr.io/edithatogo/healthpoint-rs/healthpoint-mcp:0.1.0`, then running `mcp-publisher publish server.json`. |
| Smithery | URL publishing requires a public HTTPS MCP endpoint using Streamable HTTP and OAuth support if auth is required; Smithery scans tools/prompts/resources; static server-card metadata can be served at `/.well-known/mcp/server-card.json` if scanning cannot complete. Local stdio servers publish through an MCPB bundle. Source: Smithery publish docs. | Current server is stdio-only and BYO-key, not a public Streamable HTTP endpoint or MCPB bundle. | Requires either a hosted Streamable HTTP wrapper with safe credential handling, or building/publishing an MCPB bundle. |
| Glama | Open-source server submission is from a GitHub repository with repository URL, display name, and short description; Glama performs automated quality checks including license detection, security scan, and health test; optional `glama.json` can control metadata/build/env. Connectors require HTTPS Streamable HTTP and optional test credentials unless OAuth dynamic client registration is implemented. Source: Glama FAQ. | Public GitHub repo and license are ready; stdio local server may need Glama build metadata. Hosted connector path is not ready because no public HTTPS Streamable HTTP endpoint exists. | Submit GitHub repo as open-source server after crates.io publish, or add `glama.json` if indexing needs explicit build/install metadata. |
| PulseMCP | The observed `/use-cases/submit` page says new use-case submissions are closed, while site navigation still exposes Submit server/client. Source: PulseMCP submit page. | No reliable public automated server submission requirement was available from the checked pages. | Treat as manual/contact-gated; attempt via site Submit server/client or contact/Discord after crates.io publication. |
| mcp.so | Submit form requires sign-in and fields for Type, Name, URL, and optional Server Config. Source: mcp.so submit page. | GitHub release/repo URL and server config are available; account sign-in is required. | Manual account-gated form submission after crates.io publication. |

## Requirement checklist

- [x] Public GitHub repository.
- [x] Apache-2.0 license.
- [x] GitHub Release `v0.1.0` with Linux/macOS/Windows artifacts and checksums.
- [x] Official MCP Registry-shaped `server.json`.
- [x] GitHub-scoped server name `io.github.edithatogo/healthpoint-rs`.
- [x] OCI package entry for `ghcr.io/edithatogo/healthpoint-rs/healthpoint-mcp:0.1.0`.
- [x] Dockerfile sets `io.modelcontextprotocol.server.name`.
- [x] Stdio transport declared.
- [x] `HEALTHPOINT_API_KEY` declared secret.
- [x] Visible Cargo ownership token in README: `mcp-name: io.github.edithatogo/healthpoint-rs`.
- [x] `healthpoint-mcp` published to crates.io.
- [x] Official `mcp-publisher` installed and authenticated.
- [ ] `ghcr.io/edithatogo/healthpoint-rs/healthpoint-mcp:0.1.0` published.
- [ ] Official MCP Registry publish completed and verified via registry API search.
- [ ] Smithery-compatible MCPB bundle or public Streamable HTTP endpoint prepared.
- [ ] Glama submission completed or `glama.json` added if required by indexing.
- [ ] PulseMCP submission path confirmed manually.
- [ ] mcp.so account-gated form submitted.

## Verification loop

1. Run `scripts/check-release-version.py --tag vX.Y.Z`.
2. Run `scripts/check-mcp-registry-submission.py`.
3. Publish crates in dependency order.
4. Verify `healthpoint-mcp` exists on crates.io at the release version.
5. Build and publish the GHCR OCI image tagged with the release version.
6. Run `mcp-publisher login github`.
7. Run `mcp-publisher publish server.json`.
8. Verify official registry search for `io.github.edithatogo/healthpoint-rs`.
9. Submit account-gated directories and record accepted/rejected/pending status in issue #21.
