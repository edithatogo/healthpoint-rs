# Static preflight

`scripts/static-preflight.py` is the repository's metadata-only safety net. It is intended to run in lightweight sandboxes, GitHub Actions metadata jobs, and Conductor setup before Cargo is available.

It checks:

- JSON and TOML parseability;
- workspace member shape;
- Conductor checkpoint/state consistency;
- expected script presence and executable bits;
- high-signal secret-shaped strings;
- synthetic fixture hygiene;
- `server.json` MCP secret metadata;
- rough Rust delimiter balance.

It does **not** replace Rust validation. After installing Rust, still run:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check
```
