## Summary

## Validation

- [ ] `scripts/static-preflight.py`
- [ ] `cargo fmt --all --check`
- [ ] `cargo check --workspace --all-targets --locked`
- [ ] `cargo test --workspace`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo deny check`

## Safety / licensing

- [ ] No API keys, live credentials, real Healthpoint payloads, screenshots, traces, or logs are committed.
- [ ] Any export/data changes preserve local-only defaults unless terms explicitly permit more.
- [ ] Any live API findings are redacted to endpoint shape and metadata only.
- [ ] Conductor tracks/state/checkpoints were updated when scope changed.

## Notes
