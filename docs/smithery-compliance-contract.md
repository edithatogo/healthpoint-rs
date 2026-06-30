# Smithery Compliance Contract

This contract is derived from:

- [Smithery build docs](https://smithery.ai/docs/build)
- [Smithery publish docs](https://smithery.ai/docs/build/publish)
- [Smithery triggers docs](https://smithery.ai/docs/build/triggers)
- [SlowMist MCP Security Checklist](https://github.com/slowmist/MCP-Security-Checklist)

It is written so `healthpoint-rs` can be checked locally without depending on the live
registry score. Other MCP repos can reuse it as a template.

## Contract scope

- Target a local stdio MCP server packaged as MCPB.
- Keep the contract fail-closed: if a control cannot be proven locally, classify it
  as `external_gate` rather than assuming success.
- `healthpoint-rs` is read-only. It must not claim write, publish, trigger, or event
  delivery capabilities unless a later track implements them end to end.

## Classification model

Every control is classified as one of:

- `implemented`
- `not_applicable`
- `external_gate`

Definitions:

- `implemented` means the repo itself proves the control.
- `not_applicable` means the control is outside the current server model.
- `external_gate` means the control depends on a registry account, DNS, paid plan,
  hosted approval, or other non-repo state.


## Smithery scoring rubric mirrored locally

The live Smithery frontend scoring model observed on 2026-06-30 allocates:

- Capability Quality: 40 points, covering Descriptions, Parameter descriptions, Output schemas, Tool annotations, and Naming.
- Server Metadata: 35 points, covering Description, Homepage URL, Icon, and Display name.
- Configuration UX: 25 points, covering Optional configuration and Configuration schema.

The local checker treats these as a fail-closed contract: every tool must use dot-notation naming, a meaningful description, described input parameters, a descriptive output schema, and read-only annotations; listing metadata must include homepage/repository/license/icon fields; and Smithery configuration must start in optional synthetic mode with no required fields.

## Source-derived checklist

| Area | Control | Local evidence to check | Classification for `healthpoint-rs` |
| --- | --- | --- | --- |
| Build | The server can be packaged as MCPB from a repo-controlled source of truth | `scripts/package-mcpb`, `packaging/mcpb/manifest-metadata.json` | implemented |
| Build | Generated bundle excludes secrets, payloads, traces, and caches | `scripts/check-smithery-compliance.py`, bundle contents | implemented |
| Build | Manifest version is current and valid | `manifest_version: "0.3"` in the generated bundle | implemented |
| Build | Tool metadata is complete, dot-notation named, annotated, and matches the actual MCP surface | 10 tools using dot-notation names with Parameter descriptions, Tool annotations, and output schemas in `packaging/mcpb/manifest-metadata.json` and `crates/healthpoint-mcp/src/main.rs` | implemented |
| Build | Resource metadata is complete and matches the actual MCP surface | 3 resources in metadata and server implementation | implemented |
| Build | Resource-template metadata is complete and matches the actual MCP surface | 4 resource templates in metadata and server implementation | implemented |
| Build | Prompt metadata is complete and matches the actual MCP surface | 2 prompts in metadata and server implementation | implemented |
| Build | Sensitive user config is marked sensitive and optional | `healthpoint_api_key` is optional/sensitive and `healthpoint_mode` defaults to synthetic in `packaging/mcpb/manifest-metadata.json` | implemented |
| Build | README includes a Smithery backlink/badge | `README.md` | implemented |
| Verification | Smithery backlink scan can find the accepted server URL or badge URL | `README.md` contains `https://smithery.ai/servers/edithatogo/healthpoint-rs` and `https://smithery.ai/badge/edithatogo/healthpoint-rs` | implemented |
| Publish | Local stdio servers should use MCPB bundles | Repo packaging path and CI bundle artifact | implemented |
| Publish | URL-published servers must expose Streamable HTTP | No Streamable HTTP server is published by this repo | not_applicable |
| Publish | URL-published servers requiring auth must support OAuth | No URL-published server is currently provided | not_applicable |
| Publish | If Smithery scanning cannot complete, a static server card should exist at `/.well-known/mcp/server-card.json` | No hosted HTTP surface is published today | not_applicable |
| Publish | Public listing and registry settings are complete | Smithery account state and listing visibility | external_gate |
| Publish | Exact-host DNS TXT verification is complete when a controlled homepage host is used | DNS and owned domain state | external_gate |
| Verification | Smithery exact-host TXT value `smithery-verification=cd3f0c4373ae3d6779a01d4ddd2930dfc51c9bcc932f2735377abdc5d784c2b1` is present | `github.com` TXT verification is not repo-controllable because the current Smithery homepage host is GitHub's apex domain, not a domain owned by this repo | external_gate |
| Triggers | The server advertises `ai.smithery/events` only if it really implements triggers | `crates/healthpoint-mcp/src/main.rs` capabilities | not_applicable |
| Triggers | Event list/subscribe/unsubscribe handlers exist | No trigger handlers in the server | not_applicable |
| Triggers | Delivery forwarding, signing, and payload schema are implemented | No trigger delivery surface exists | not_applicable |
| Security | Inputs are validated and constrained | Rust schemas in `crates/healthpoint-mcp/src/main.rs` and client-side clamps | implemented |
| Security | Output is redacted and secret-safe | Diagnostic and access-note handlers never return the API key | implemented |
| Security | Least privilege is preserved | Read-only tools only; no writes or mutations | implemented |
| Security | Dependency and bundle integrity are checked | `cargo` workspace, bundle validation, static checks | implemented |
| Security | Logging does not leak secrets or payloads | No secret-bearing logs or exports in CI/bundle paths | implemented |
| Security | Prompt injection risk is acknowledged and bounded | Safe-search and license-check prompts are explicit and read-only | implemented |
| Security | Access control and license constraints are documented | Access policy and license-check prompt | implemented |
| Security | Rate limiting / result limiting is applied where the upstream API allows it | `limit` bounds and query clamping in code | implemented |
| Security | Platform compatibility is considered | Linux, macOS, and Windows bundle jobs in CI | implemented |
| Security | Anything dependent on customer approval or external contracts is clearly separated | Healthpoint API license and Smithery account gates | external_gate |

## Required evidence for local assessment

- `scripts/check-smithery-compliance.py` passes and enforces Capability Quality, Server Metadata, and Configuration UX controls.
- `scripts/validate-mcp-server fast` passes.
- `scripts/package-mcpb` produces a bundle that matches the declared surface.
- The bundle does not include `.env`, API keys, tokens, live payloads, traces, or logs.
- CI can build the bundle on Linux, macOS, and Windows.
- The registry score, if referenced, is treated as advisory evidence only.

## Live Smithery verification status on 2026-06-30

- Backlink: repo-controlled and implemented. `README.md` contains both accepted
  backlink forms: the server URL and the badge URL. If Smithery still reports the
  backlink missing, rerun the Smithery verification check or set the optional
  custom backlink URL to `https://github.com/edithatogo/healthpoint-rs`.
- TXT record: external/manual gate. Smithery is currently checking host
  `github.com` for
  `smithery-verification=cd3f0c4373ae3d6779a01d4ddd2930dfc51c9bcc932f2735377abdc5d784c2b1`.
  `github.com` TXT verification is not repo-controllable.
  This cannot be satisfied from this repository because the repo owner cannot add
  TXT records to GitHub's apex domain. To close it, change Smithery's homepage to
  a controlled host such as a GitHub Pages custom domain, then add the TXT value
  at that host's DNS provider.

## Notes on applicability

- SlowMist items are not all applicable to a read-only stdio server.
- Triggers are out of scope unless the server explicitly advertises and implements
  the Smithery event extension.
- Publish-time approvals, paid plans, and DNS verification are external gates and
  should never be represented as repo bugs.

## MCP-Manager checklist themes

The MCP-Manager checklist is treated as an operational security reference. The
following themes are relevant to this repo and to other MCP servers using this
contract:

- Tool selection guidance should be explicit in prompts and docs so the model does
  not have to infer which tool to use from ambiguous names alone.
- Logging and auditing should be structured, searchable, and safe to retain without
  storing secrets or live payloads.
- Deployment guidance should clearly distinguish local stdio servers from hosted
  HTTP servers and should recommend sandboxing where possible.
- Authentication and authorization should be treated as external gates when they
  depend on a registry, host identity, or a user-managed provider.
- Security threat themes to account for include prompt injection, tool poisoning,
  rug-pull updates, retrieval-agent deception, cross-server shadowing, server
  spoofing, token theft, and account takeover.
- For a read-only server, the best mitigations are least privilege, explicit tool
  descriptions, strict input validation, secret redaction, bounded outputs, and
  visible provenance.

These themes are already reflected in the repo where they can be proven locally:

- `healthpoint_safe_search` and `healthpoint_license_check` constrain user intent.
- `scripts/check-smithery-compliance.py` enforces the manifest, README, and bundle
  invariants.
- `docs/smithery-compliance-playbook.md` separates repo-controlled controls from
  external gates.
- `healthpoint-rs` does not advertise Smithery trigger support.
