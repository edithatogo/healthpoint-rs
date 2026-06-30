# Implementation Plan

## Phase 1: Track setup and baseline

- [x] Task: Create Conductor track artifacts.
    - [x] Add `metadata.json`.
    - [x] Add `spec.md`.
    - [x] Add `plan.md`.
    - [x] Add `index.md`.
- [x] Task: Record baseline Smithery state.
    - [x] Record score `51/100`.
    - [x] Record accepted Smithery releases.
    - [x] Record paid/DNS as external gates.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Track setup and baseline' (Protocol in workflow.md)

## Phase 2: Local MCP validation

- [x] Task: Add preferred CLI validator script.
    - [x] Add `scripts/validate-mcp-server`.
    - [x] Default to fast mode with `--skip-mcp-scan`.
    - [x] Support full mode when `mcp-scan` is available.
- [x] Task: Add ignored report path.
    - [x] Write reports under `target/mcp-validation/`.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Local MCP validation' (Protocol in workflow.md)

## Phase 3: Manifest and package quality

- [x] Task: Add MCPB metadata source of truth.
    - [x] Add `packaging/mcpb/manifest-metadata.json`.
    - [x] Include 10 tools, 3 resources, 4 templates, and 2 prompts.
- [x] Task: Refactor MCPB packaging.
    - [x] Generate `manifest.json` from metadata.
    - [x] Keep secrets as user config placeholders.
- [x] Task: Add static Smithery compliance checker.
    - [x] Check README badge.
    - [x] Check manifest completeness.
    - [x] Check bundle contents and secret patterns.
- [x] Task: Conductor - User Manual Verification 'Phase 3: Manifest and package quality' (Protocol in workflow.md)

## Phase 4: Multi-platform release automation

- [x] Task: Add Smithery compliance workflow.
    - [x] Build Linux, macOS, and Windows MCPB bundles.
    - [x] Upload bundle artifacts.
    - [x] Upload MCP validation reports.
    - [x] Publish to Smithery only on manual dispatch with `SMITHERY_TOKEN`.
- [x] Task: Conductor - User Manual Verification 'Phase 4: Multi-platform release automation' (Protocol in workflow.md)

## Phase 5: Reusable cross-repo playbook

- [x] Task: Add reusable Smithery playbook.
    - [x] Document repo-controlled checks.
    - [x] Document external gates.
    - [x] Document validator choices.
- [x] Task: Update registry docs.
    - [x] Record score state and validation strategy.
- [x] Task: Conductor - User Manual Verification 'Phase 5: Reusable cross-repo playbook' (Protocol in workflow.md)

## Phase 6: Verification and closeout

- [x] Task: Run local validation.
    - [x] Run `scripts/package-mcpb`.
    - [x] Run `scripts/check-smithery-compliance.py`.
    - [x] Run `scripts/validate-mcp-server fast` with the validator installed in an isolated Python 3.13 venv.
- [x] Task: Publish improved Smithery release when appropriate.
    - [x] Use local CLI to publish the hardened bundle.
    - [x] Record deployment ID `76edfcf1-d617-42fe-8b4e-06c6e3917854`.
    - [ ] Record score after release once Smithery verification page loads reliably.
- [ ] Task: Open PR and let branch protection run checks.
- [ ] Task: Conductor - User Manual Verification 'Phase 6: Verification and closeout' (Protocol in workflow.md)
