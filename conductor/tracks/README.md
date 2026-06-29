# Conductor Tracks

This directory is reserved for longer track specs when a track grows beyond the summary in `../tracks.md`.

Status labels:

- **Implemented**: wired into CLI/MCP/source paths and covered by tests or documented manual validation.
- **Partial**: useful behaviour exists, but validation, endpoint details, or release hardening remain incomplete.
- **Probe-only**: code tests likely endpoints or parameters and reports graceful unsupported/not-found responses.
- **Read-only**: inspection/retrieval exists, but mutation is absent by design.
- **Planned**: tracked but not substantially implemented.

Operating rules:

- Update `conductor/state.json` after each meaningful implementation pass.
- Update `conductor/checkpoints/` before handing off to another agent.
- Keep API keys, real Healthpoint responses, local caches, and generated exports out of Git.
- Record endpoint-shape discoveries as metadata, not payloads.
