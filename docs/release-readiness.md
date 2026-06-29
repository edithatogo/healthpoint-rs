# Release readiness checklist

Before first public release:

- [x] Validate workspace compile/test on Rust stable.
- [x] Add cross-platform Rust CI on Linux, macOS, and Windows.
- [x] Add MSRV check for Rust 1.88.
- [x] Pin `rmcp` dependency to a release/tag/commit instead of git-main.
- [x] Validate Healthpoint auth scheme against licensed docs.
- [x] Confirm FHIR search parameter names.
- [x] Add mock-server tests for paging, errors, and auth redaction.
- [x] Review Healthpoint terms for caching, bulk export, attribution, and redistribution.
- [x] Add binary release workflow.
- [x] Add crate-level docs and examples.
- [x] Add MCP resource templates or explicitly defer them after RMCP validation.
- [x] Add shell completions.
- [x] Decide whether `healthpoint-osd-adapter` remains internal or becomes a published crate.
- [x] Confirm generated data files are excluded from release artifacts.
- [x] Generate and commit `Cargo.lock` from a Rust-enabled environment.
- [x] Run `cargo deny check` after dependency resolution.
- [x] Enable daily dependency automation for Cargo and GitHub Actions.

## Release posture

The first release is a BYO-key SDK/CLI/MCP/tooling release, not a public Healthpoint data release. Generated exports, JSONL, CSV, Parquet, logs, traces, HAR files, local databases, `.env`, and `.healthpoint/` contract evidence are excluded by `.gitignore` and must not be attached to release artifacts.

`healthpoint-osd-adapter` remains an internal adapter crate in this workspace until Healthpoint gives written approval for any public/open-data publication or redistribution use. The public artifact may describe the adapter shape, but must not publish Healthpoint-derived datasets.

Dylan/VUW production API use is only covered for the executed academic non-commercial research purpose and its operational limits. Public hosted tools, shared API-key services, data mirrors, and open-data releases remain out of scope.
