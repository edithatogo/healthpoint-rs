# MCP registry submissions

This document records the current submission requirements checked on 2026-06-30 and the project-specific status for `healthpoint-rs`. The canonical official-registry validator is `scripts/check-mcp-registry-submission.py`; Smithery/MCPB compliance is checked by `scripts/check-smithery-compliance.py`, and protocol validation is wrapped by `scripts/validate-mcp-server`.

## Project submission identity

- Official MCP Registry server name: `io.github.edithatogo/healthpoint-rs`.
- OCI package to register: `ghcr.io/edithatogo/healthpoint-mcp:0.1.0`.
- Rust package distribution path: `healthpoint-mcp` on crates.io.
- Transport: `stdio`.
- Distribution path: GHCR OCI image for the official MCP Registry, crates.io for Rust users, and GitHub Release binaries as additional convenience distributions.
- Secret configuration: `HEALTHPOINT_API_KEY` is optional and marked secret; `HEALTHPOINT_MODE` defaults to synthetic for no-credential installability.
- Safety boundary: BYO-key, read-only tools/resources/prompts, no bundled Healthpoint data, no public redistribution of licensed Healthpoint payloads.

## Registry matrix

| Registry | Current requirements checked | healthpoint-rs status | Submission gate |
| --- | --- | --- | --- |
| Official MCP Registry | Uses `server.json`; registry only stores metadata; package must be published first; publisher CLI is `mcp-publisher`; GitHub auth requires an `io.github.<user>/...` server name. Live publisher validation currently rejects `registryType: cargo` and accepts supported package ownership mechanisms such as OCI. OCI images require the `io.modelcontextprotocol.server.name` image label, and OCI package versions must be encoded in `identifier` rather than a separate `version` field. Sources: official quickstart/package-types docs and live `mcp-publisher publish` validation on 2026-06-30. | Complete. `ghcr.io/edithatogo/healthpoint-mcp:0.1.0` is public and `mcp-publisher publish server.json` succeeded for `io.github.edithatogo/healthpoint-rs` version `0.1.0`. Crates.io remains published for Rust users. | Closed. |
| Smithery | URL publishing requires a public HTTPS MCP endpoint using Streamable HTTP and OAuth support if auth is required; Smithery scans tools/prompts/resources; static server-card metadata can be served at `/.well-known/mcp/server-card.json` if scanning cannot complete. Local stdio servers publish through an MCPB bundle. Source: Smithery publish docs. | Complete for initial listing and now hardened for repo-controlled compliance. The MCPB manifest is generated from `packaging/mcpb/manifest-metadata.json`, covers 10 dot-notation tools with parameter descriptions, output schemas, annotations, 3 resources, 4 resource templates, and 2 prompts, and is checked by `scripts/check-smithery-compliance.py`. Protocol validation is wrapped by `scripts/validate-mcp-server` using the `mcp-validation` CLI. A local fast validation pass on 2026-06-30 reported valid protocol compliance, 10 tools, 2 prompts, 3 resources, ping support, error compliance, repository availability, license, and registry checks. Accepted deployments now include `c73eb36e-66ba-4d28-b95e-71b92dcf20f2`, `f68c78e1-5eec-4c4b-a530-c34291c84819`, `76edfcf1-d617-42fe-8b4e-06c6e3917854`, `e82b30c8-d456-4776-8f73-5913876668cf`, `d57200f6-ab14-4369-8a14-67fe718f52b8`, `544095f0-09eb-41d4-8e77-2606580ad9eb`, `677ad41a-384f-40d6-9ce0-b02774bb7294`, `36ec9477-599d-4d76-a900-f37d83753746`, and `dda10c87-9df1-4f17-9f4d-0b9bb5c950ba`. The visible Smithery quality score now reads `98/100` in the dashboard, but the last `2` points are not identifiable from public HTML/API surfaces in this shell. Public API recheck shows `required: []` for config, `10/10` tools expose `outputSchema`, the runtime bundle is accepted, and the icon URL is reachable. | Closed for initial Smithery listing and repo-controlled quality remediation. Remaining verification blockers are dashboard score confirmation for the last `2` points, Smithery listing metadata account update if the API continues to ignore `homepage`/`repositoryUrl`/`license` fields, exact-homepage-host TXT record, and paid developer plan. |
| Glama | Open-source server submission is from a GitHub repository with repository URL, display name, short description, release metadata, README, license, maintenance/security signals, `glama.json` maintainer metadata, related servers, and a generated tool-quality score. The visible score page says the quality score is `70% Tool Definition Quality + 30% Server Coherence`; Tool Definition Quality is `60% mean TDQS + 40% minimum TDQS` across Purpose Clarity, Usage Guidelines, Behavioral Transparency, Parameter Semantics, Conciseness/Structure, and Contextual Completeness. Server Coherence scores Disambiguation, Naming Consistency, Tool Count Appropriateness, and Completeness equally. Source: live Glama score page checked 2026-06-30. | Posted with Glama release `v0.1.0` created from successful test `019f181f-0b4a-7200-b5a0-f52c6516b629` on 2026-06-30. Try in Browser launched Glama instance `o7bzzvjw41` and successfully executed `healthpoint_access_policy`. Public/authenticated score observations differ (`58%` in the user-visible flow after first release; `83%` in the public score page fetched from this shell). Repo-controlled remediation now includes `glama.json` with `maintainers: ["edithatogo"]` and expanded runtime MCP tool descriptions that cover usage guidance, alternatives, read-only/BYO-key behavior, limits, pagination/error behavior, and JSON provenance/access return shape. `scripts/check-mcp-registry-submission.py` emits a `glama_score_contract` block with the rubric and observed state. Hosted connector path is not ready because no public HTTPS Streamable HTTP endpoint exists. | Merge these changes to `main`, trigger Glama sync/release refresh, and re-check both public and authenticated score views. Related servers remains manual/external directory curation unless Glama exposes an editable repo-backed field. |
| PulseMCP | The public `/api` page says PulseMCP ingests the Official MCP Registry daily and processes it weekly. The `/submit` page only offers MCP Server or MCP Client entry points and then routes to a manual email gate asking the user to contact `hello@pulsemcp.com` for registry publication or adjustments. | Repo-ready but external-gated. `healthpoint-rs` is already published to the Official MCP Registry, so PulseMCP should eventually ingest it on its own schedule. | Monitor for ingestion; if not visible after a week, email `hello@pulsemcp.com` with the official registry entry URL and request inclusion. |
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
- [x] `HEALTHPOINT_API_KEY` declared optional secret and synthetic mode declared as default.
- [x] Visible Cargo ownership token in README: `mcp-name: io.github.edithatogo/healthpoint-rs`.
- [x] `healthpoint-mcp` published to crates.io.
- [x] Official `mcp-publisher` installed and authenticated.
- [x] `ghcr.io/edithatogo/healthpoint-mcp:0.1.0` published and public.
- [x] Official MCP Registry publish completed for `io.github.edithatogo/healthpoint-rs` version `0.1.0`.
- [x] Smithery-compatible MCPB packaging recipe prepared.
- [x] Smithery Darwin/arm64 MCPB bundle built and submitted from logged-in account.
- [x] Smithery MCPB metadata source covers tools/resources/templates/prompts.
- [x] Smithery compliance contract added and checked against Smithery/SlowMist/MCP-Manager themes.
- [x] Smithery static compliance checker added.
- [x] MCP protocol validator wrapper added for `mcp-validation`.
- [x] Multi-platform MCPB build workflow added.
- [x] Glama repo listing posted, Glama release `v0.1.0` created from successful test `019f181f-0b4a-7200-b5a0-f52c6516b629`, and Try in Browser executed `healthpoint_access_policy`.
- [x] Local `glama.json` added with required `maintainers` metadata and validated by `scripts/check-mcp-registry-submission.py`.
- [x] MCP tool descriptions expanded against Glama TDQS dimensions.
- [ ] Glama live page has re-synced from `main` and no longer reports `No glama.json`.
- [ ] Glama related servers configured if a suitable related-server set is identified.
- [ ] PulseMCP ingestion observed or manual email follow-up sent after the official-registry wait window.
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
