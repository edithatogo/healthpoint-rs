# Release readiness checklist

Before first public release:

- [x] Validate workspace compile/test on Rust stable.
- [x] Add cross-platform Rust CI on Linux, macOS, and Windows.
- [x] Add MSRV check for Rust 1.88.
- [x] Pin `rmcp` dependency to a release/tag/commit instead of git-main.
- [ ] Validate Healthpoint auth scheme against licensed docs.
- [ ] Confirm FHIR search parameter names.
- [ ] Add mock-server tests for paging, errors, and auth redaction.
- [ ] Review Healthpoint terms for caching, bulk export, attribution, and redistribution.
- [x] Add binary release workflow.
- [ ] Add crate-level docs and examples.
- [ ] Add MCP resource templates or explicitly defer them after RMCP validation.
- [ ] Add shell completions.
- [ ] Decide whether `healthpoint-osd-adapter` remains internal or becomes a published crate.
- [ ] Confirm generated data files are excluded from release artifacts.
- [x] Generate and commit `Cargo.lock` from a Rust-enabled environment.
- [x] Run `cargo deny check` after dependency resolution.
- [x] Enable daily dependency automation for Cargo and GitHub Actions.
