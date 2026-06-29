# Roadmap

## Now

- Get the workspace through first Rust-enabled CI: format, check, clippy, tests, deny.
- Validate `rmcp` API surface and pin dependency away from `branch = "main"` before release.
- Confirm Healthpoint authentication, base URL, search parameters, nearby semantics, pagination, and direct reads using Dylan's licensed API key.

## Next

- Add mock-server integration tests in Rust.
- Add native MCP resources and resource templates.
- Add shell completions and manpage generation.
- Add Arrow/Parquet export once JSONL/CSV contracts are proven.
- Turn redacted live contract findings into policy and API-assumption updates.

## Later

- Implement open_social_data integration only after terms review permits the intended data handling.
- Extract shared provenance/access-policy crates if multiple edithatogo tools converge on the same contracts.
- Add release automation, binary provenance, and MCP registry packaging.
