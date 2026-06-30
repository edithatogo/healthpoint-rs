# MCP registry submissions

This document records the current submission requirements checked on 2026-06-30 and the project-specific status for `healthpoint-rs`. The canonical local validator is `scripts/check-mcp-registry-submission.py`; the GitHub workflow is `.github/workflows/mcp-registry-submission.yml`.

## Project submission identity

- Official MCP Registry server name: `io.github.edithatogo/healthpoint-rs`.
- OCI package to register: `ghcr.io/edithatogo/healthpoint-mcp:0.1.0`.
- Rust package distribution path: `healthpoint-mcp` on crates.io.
- Transport: `stdio`.
- Distribution path: GHCR OCI image for the official MCP Registry, crates.io for Rust users, and GitHub Release binaries as additional convenience distributions.
- Secret configuration: `HEALTHPOINT_API_KEY` is required and marked secret.
- Safety boundary: BYO-key, read-only tools/resources/prompts, no bundled Healthpoint data, no public redistribution of licensed Healthpoint payloads.

## Registry matrix

| Registry | Current requirements checked | healthpoint-rs status | Submission gate |
| --- | --- | --- | --- |
| Official MCP Registry | Uses `server.json`; registry only stores metadata; package must be published first; publisher CLI is `mcp-publisher`; GitHub auth requires an `io.github.<user>/...` server name. Live publisher validation currently rejects `registryType: cargo` and accepts supported package ownership mechanisms such as OCI. OCI images require the `io.modelcontextprotocol.server.name` image label, and OCI package versions must be encoded in `identifier` rather than a separate `version` field. Sources: official quickstart/package-types docs and live `mcp-publisher publish` validation on 2026-06-30. | Complete. `ghcr.io/edithatogo/healthpoint-mcp:0.1.0` is public and `mcp-publisher publish server.json` succeeded for `io.github.edithatogo/healthpoint-rs` version `0.1.0`. Crates.io remains published for Rust users. | Closed. |
| Smithery | URL publishing requires a public HTTPS MCP endpoint using Streamable HTTP and OAuth support if auth is required; Smithery scans tools/prompts/resources; static server-card metadata can be served at `/.well-known/mcp/server-card.json` if scanning cannot complete. Local stdio servers publish through an MCPB bundle. Source: Smithery publish docs. | Complete for Darwin/arm64 local bundle. `scripts/package-mcpb` built `target/mcpb/healthpoint-rs-0.1.0-darwin-arm64.mcpb`, and `smithery mcp publish ... -n edithatogo/healthpoint-rs` returned deployment `c73eb36e-66ba-4d28-b95e-71b92dcf20f2`, status `SUCCESS`, and `mcpUrl` `https://healthpoint-rs--edithatogo.run.tools`. Smithery settings were updated on 2026-06-30 with description, homepage, GitHub repository, and public listing; observed score improved from `27/100` to `51/100`. The Smithery badge backlink was added to `README.md`, and follow-up release `f68c78e1-5eec-4c4b-a530-c34291c84819` was accepted. | Closed for initial Smithery listing. Remaining verification blockers are quality score >80, exact-homepage-host TXT record, and paid developer plan. Future work: publish additional platform bundles if Smithery needs separate Linux/Windows local artifacts. |
| Glama | Open-source server submission is from a GitHub repository with repository URL, display name, and short description; Glama performs automated quality checks including license detection, security scan, and health test; optional `glama.json` can control metadata/build/env. Connectors require HTTPS Streamable HTTP and optional test credentials unless OAuth dynamic client registration is implemented. Source: Glama FAQ. | Public GitHub repo and license are ready; stdio local server may need Glama build metadata. Hosted connector path is not ready because no public HTTPS Streamable HTTP endpoint exists. | Submit GitHub repo as open-source server after crates.io publish, or add `glama.json` if indexing needs explicit build/install metadata. |
| PulseMCP | The observed `/use-cases/submit` page says new use-case submissions are closed, while site navigation still exposes Submit server/client. CLI access to `https://www.pulsemcp.com/submit` is blocked by Cloudflare. PulseMCP also indexes the Official MCP Registry, so the published official registry entry is the primary route for eventual listing. | No reliable public automated server submission requirement is available from checked pages. Browser/manual submission remains unclear because the user could not locate login and Cloudflare blocks non-browser inspection. | Manual/contact-gated; use the website, Discord/contact route, or wait for Official MCP Registry ingestion. |
| mcp.so | Submit form requires sign-in and fields for Type, Name, URL, and optional Server Config. The form posts to `/api/submit-project` and rejects unauthenticated API submissions with `no auth, please login`. Source: mcp.so submit app bundle and live API response on 2026-06-30. | Submitted from the logged-in browser session on 2026-06-30. The created server ID is `9b528cf0-6c30-4566-84e4-8e8ac43070cb`; the management URL is `https://mcp.so/my-servers/9b528cf0-6c30-4566-84e4-8e8ac43070cb/edit`; current observed listing status is `created`. | Closed for initial submission; monitor for publication/indexing status changes. |

## Requirement checklist

- [x] Public GitHub repository.
- [x] Apache-2.0 license.
- [x] GitHub Release `v0.1.0` with Linux/macOS/Windows artifacts and checksums.
- [x] Official MCP Registry-shaped `server.json`.
- [x] GitHub-scoped server name `io.github.edithatogo/healthpoint-rs`.
- [x] OCI package entry for `ghcr.io/edithatogo/healthpoint-mcp:0.1.0`.
- [x] Dockerfile sets `io.modelcontextprotocol.server.name`.
- [x] Stdio transport declared.
- [x] `HEALTHPOINT_API_KEY` declared secret.
- [x] Visible Cargo ownership token in README: `mcp-name: io.github.edithatogo/healthpoint-rs`.
- [x] `healthpoint-mcp` published to crates.io.
- [x] Official `mcp-publisher` installed and authenticated.
- [x] `ghcr.io/edithatogo/healthpoint-mcp:0.1.0` published and public.
- [x] Official MCP Registry publish completed for `io.github.edithatogo/healthpoint-rs` version `0.1.0`.
- [x] Smithery-compatible MCPB packaging recipe prepared.
- [x] Smithery Darwin/arm64 MCPB bundle built and submitted from logged-in account.
- [ ] Glama submission completed or `glama.json` added if required by indexing.
- [ ] PulseMCP submission path confirmed manually or listing observed via Official MCP Registry ingestion.
- [x] mcp.so account-gated form submitted from logged-in browser session.

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
