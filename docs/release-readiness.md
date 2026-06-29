# Release readiness checklist

Before first public release:

- [ ] Validate workspace compile/test on Rust stable.
- [ ] Pin `rmcp` dependency to a release/tag or document why git-main remains required.
- [ ] Validate Healthpoint auth scheme against licensed docs.
- [ ] Confirm FHIR search parameter names.
- [ ] Add mock-server tests for paging, errors, and auth redaction.
- [ ] Review Healthpoint terms for caching, bulk export, attribution, and redistribution.
- [ ] Add `cargo dist` or equivalent binary release plan.
- [ ] Add crate-level docs and examples.
- [ ] Add MCP resource templates or explicitly defer them after RMCP validation.
- [ ] Add shell completions.
- [ ] Decide whether `healthpoint-osd-adapter` remains internal or becomes a published crate.
- [ ] Confirm generated data files are excluded from release artifacts.
- [ ] Generate and commit `Cargo.lock` from a Rust-enabled environment.
- [ ] Run `cargo deny check` after dependency resolution.
