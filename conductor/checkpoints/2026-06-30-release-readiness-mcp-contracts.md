# Release readiness, MCP resources, and contract coverage checkpoint

Date: 2026-06-30

## Completed

- Added ignored local contract evidence staging at `.healthpoint/contracts/`.
- Retrieved the June 2026 Healthpoint API and Data Licence variation from VUW Outlook via Microsoft Graph CLI auth and stored raw messages/attachments under ignored local evidence.
- Updated machine-readable Healthpoint access policy with the contract-backed academic non-commercial research scope, attribution requirement, production limits, and AI/public-data restrictions.
- Implemented native MCP resources, resource templates, and prompts alongside existing read-only tools.
- Added client mock coverage for paging metadata, auth header handling, and API-key redaction on errors.
- Updated release readiness and MCP registry readiness documentation.
- Validated with `cargo fmt --all --check`, `cargo check --workspace --all-targets --locked`, `cargo test --workspace --locked`, and `cargo clippy --workspace --all-targets --locked -- -D warnings`.
- Ran an MCP JSON-RPC client smoke against the synthetic server; resources, resource templates, prompts, policy read, and query resource read succeeded.

## Remaining gates

- Use only metadata-only live captures; never commit real Healthpoint API payloads.
- Obtain Healthpoint written approval before public-cache, public-data, redistribution, open-data, AI dataset, hosted proxy, or non-research commercial usage.
