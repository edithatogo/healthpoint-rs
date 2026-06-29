# Conductor context management

This repository is designed to be resumed by a person or an agent without needing private chat history.

## Context files

| File | Purpose |
| --- | --- |
| `conductor/brief.md` | Stable one-page project brief and safety boundary. |
| `conductor/state.json` | Machine-readable status, decisions, blockers, and next tracks. |
| `conductor/tracks.md` | Human-readable status matrix and task list. |
| `conductor/tracks/*.md` | Per-track notes that can be updated independently. |
| `conductor/checkpoints/*.md` | Dated handoff snapshots. |
| `conductor/decisions/*.md` | ADR-style architectural decisions. |
| `workflow.md` | Delivery workflow and acceptance rules. |
| `AGENTS.md` | Repository-specific instructions for automated agents. |

## Commands

```bash
bin/conductor-status
bin/conductor-setup
scripts/static-preflight.py
```

`bin/conductor-setup` should remain safe in metadata-only environments. Set `CONDUCTOR_ALLOW_NO_CARGO=1` to skip Cargo-dependent checks when Rust is unavailable.

## Update rules

1. When scope changes, update `conductor/tracks.md` first.
2. When a decision becomes durable, add an ADR under `conductor/decisions/`.
3. When handing off, add a checkpoint and point `conductor/state.json.latest_checkpoint` at it.
4. Never put API keys, live response bodies, logs, traces, or real Healthpoint payloads into Conductor files.
5. Record blockers concretely; do not hide environment or licensing gates.
