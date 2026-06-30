# MCP registry submissions

This document records the current submission requirements checked on 2026-06-30 and the project-specific status for `healthpoint-rs`. The canonical official-registry validator is `scripts/check-mcp-registry-submission.py`; Smithery/MCPB compliance is checked by `scripts/check-smithery-compliance.py`, and protocol validation is wrapped by `scripts/validate-mcp-server`.

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
| Smithery | URL publishing requires a public HTTPS MCP endpoint using Streamable HTTP and OAuth support if auth is required; Smithery scans tools/prompts/resources; static server-card metadata can be served at `/.well-known/mcp/server-card.json` if scanning cannot complete. Local stdio servers publish through an MCPB bundle. Source: Smithery publish docs. | Complete for initial listing and now hardened for repo-controlled compliance. The MCPB manifest is generated from `packaging/mcpb/manifest-metadata.json`, covers 10 tools, 3 resources, 4 resource templates, and 2 prompts, and is checked by `scripts/check-smithery-compliance.py`. Protocol validation is wrapped by `scripts/validate-mcp-server` using the `mcp-validation` CLI. A local fast validation pass on 2026-06-30 reported valid protocol compliance, 10 tools, 2 prompts, 3 resources, ping support, error compliance, repository availability, license, and registry checks. Existing accepted deployments are `c73eb36e-66ba-4d28-b95e-71b92dcf20f2`, `f68c78e1-5eec-4c4b-a530-c34291c84819`, and hardened release `76edfcf1-d617-42fe-8b4e-06c6e3917854`; observed score improved from `27/100` to `51/100` before this hardening pass. | Closed for initial Smithery listing. Remaining verification blockers are quality score >80, exact-homepage-host TXT record, and paid developer plan. Repo-controlled next step is to publish the hardened multi-platform bundles and re-check score. |
| Glama | Open-source server submission is from a GitHub repository with repository URL, display name, short description, release metadata, README, license, maintenance/security signals, `glama.json` maintainer metadata, related servers, and a generated tool-quality score. The visible score page says the quality score is `70% Tool Definition Quality + 30% Server Coherence`; Tool Definition Quality is `60% mean TDQS + 40% minimum TDQS` across Purpose Clarity, Usage Guidelines, Behavioral Transparency, Parameter Semantics, Conciseness/Structure, and Contextual Completeness. Server Coherence scores Disambiguation, Naming Consistency, Tool Count Appropriateness, and Completeness equally. Source: live Glama score page checked 2026-06-30. | Submitted and released. Live observed profile score is `83%` in the public score page, while the user observed `58%` in the authenticated/dashboard flow before this remediation. Latest Glama release is `v0.1.0`; public Server Coherence is `A` with `5/5` on all four dimensions; public Tool Definition Quality is `A` with average `3.7/5` across 10 tools and lowest `3.1/5`. Repo-controlled remediation now includes `glama.json` with `maintainers: ["edithatogo"]` and expanded runtime MCP tool descriptions that explicitly cover usage guidance, alternatives, read-only/BYO-key behavior, limits, pagination/error behavior, and JSON provenance/access return shape. `scripts/check-mcp-registry-submission.py` emits a `glama_score_contract` block with this rubric and observed state. | Closed for initial submission/release. Repo-controlled next step is to merge these changes to `main`, trigger Glama sync/release refresh, and re-check both public and authenticated score views. Related servers remains manual/external directory curation unless Glama exposes an editable repo-backed field. |
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
- [x] Smithery MCPB metadata source covers tools/resources/templates/prompts.
- [x] Smithery static compliance checker added.
- [x] MCP protocol validator wrapper added for `mcp-validation`.
- [x] Multi-platform MCPB build workflow added.
- [x] Glama submission completed and release `v0.1.0` observed.
- [x] Local `glama.json` added with required `maintainers` metadata and validated by `scripts/check-mcp-registry-submission.py`.
- [x] MCP tool descriptions expanded against Glama TDQS dimensions.
- [ ] Glama live page has re-synced from `main` and no longer reports `No glama.json`.
- [ ] Glama related servers configured if a suitable related-server set is identified.
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


## Smithery validator choices

- Primary CLI validator: `RHEcosystemAppEng/mcp-validation`, invoked through `scripts/validate-mcp-server`. Fast mode uses `--skip-mcp-scan`; full mode enables the validator security scan path when locally available.
- Reference only: `Janix-ai/mcp-validator`, because AGPL code must not be vendored into this Apache-2.0 repo.
- Optional/manual: Apify `rocketagro/mcp-validator`, because it is a paid cloud Actor rather than the preferred local CLI.
- Benchmark only: `punkpeye/awesome-mcp-servers`, useful for directory/readme quality comparison but not a validator.

Reports are written under ignored paths such as `target/mcp-validation/`.
