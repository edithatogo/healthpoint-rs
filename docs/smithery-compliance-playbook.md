# Smithery compliance playbook

This playbook is the reusable checklist for improving Smithery score and verification for local stdio MCP servers. It is written from the `healthpoint-rs` hardening pass but should apply to other MCP repos with minimal changes.

## Score strategy

Separate gates into two classes:

- Repo-controlled: complete metadata, README backlink, public listing, MCPB manifest quality, validator reports, multi-platform bundles, no bundled secrets.
- External: paid developer plan, DNS TXT record on an exact homepage host, account approvals, and any third-party registry moderation.

For `healthpoint-rs`, the target is maximum free/repo-controlled score. Full verification is intentionally out of scope until a paid plan and controlled homepage host are approved.

## Required repo-controlled items

- Add the Smithery badge near the top of `README.md`:

```markdown
[![smithery badge](https://smithery.ai/badge/<namespace>/<server>)](https://smithery.ai/servers/<namespace>/<server>)
```

- Set Smithery server settings:
  - Display name
  - Description
  - Homepage
  - GitHub repository
  - Public listing, not unlisted

- Generate MCPB from a metadata source of truth:
  - Include `manifest_version: "0.3"`.
  - Include complete `tools` with descriptions, `inputSchema`, and preferably `outputSchema`.
  - Include prompts where supported by the MCPB spec.
  - Record resources/resource templates in `_meta.io.modelcontextprotocol` when the bundle format has no first-class resource field.
  - Mark secret user config as sensitive and required.

- Exclude from bundles:
  - `.env` files
  - API keys and tokens
  - real API payloads
  - generated exports
  - logs, traces, HAR files, caches, databases, and parquet/csv/jsonl data

## Preferred CLI validation

Primary validator: `RHEcosystemAppEng/mcp-validation`.

Install:

```bash
python3.13 -m pip install git+https://github.com/RHEcosystemAppEng/mcp-validation.git
```

Run fast local validation:

```bash
scripts/validate-mcp-server fast
```

Run full validation including `mcp-scan` where available:

```bash
scripts/validate-mcp-server full
```

Reports belong under ignored build/output directories such as `target/mcp-validation/`.

## Secondary references

- `Janix-ai/mcp-validator`: reference only unless licensing is reviewed. Do not vendor AGPL code into Apache-2.0 projects.
- Apify `rocketagro/mcp-validator`: optional paid/manual cloud validation. Use only when an external report is worth the cost.
- `punkpeye/awesome-mcp-servers`: directory/readme benchmark, not a validator.

## CI pattern

- Build and validate the MCPB bundle on Linux, macOS, and Windows.
- Upload bundle artifacts.
- Run the CLI MCP validator and upload JSON reports.
- Publish to Smithery only from an explicit manual workflow or protected release workflow when `SMITHERY_TOKEN` is present.

## Closeout checklist

- Smithery release accepted.
- Smithery score recorded before and after changes.
- README badge visible on the default branch.
- Server is publicly listed.
- External gates documented separately from repo-controlled gaps.
