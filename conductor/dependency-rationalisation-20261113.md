# Rust dependency rationalisation — healthpoint-rs (2026-11-13)

**Status:** proposal and review contract; no code adoption, runtime verification or benchmark result is claimed here.

## Repository-specific decisions

- Reconcile existing core, FHIR, client, export, CLI, MCP, OSD adapter and testkit ownership; preserve clinical privacy boundaries.
- **Verified:** workspace already centralises Anyhow, Clap, Reqwest/Rustls, Serde, Thiserror, Tokio, Tracing and versioned internal crates.
- **Adopt sparingly:** targeted Proptest round-trip and resource-bounded FHIR/coding validation; Insta only for fully synthetic/redacted FHIR fixtures.
- **Evaluate:** Criterion for terminology matching, FHIR parse/export and bounded client paginations; HTTP client reuse instead of duplicated hand-made retries.
- **Reject:** adding Rayon, Polars, SQLx, Ratatui or PyO3 to core without concrete measured need; never emit clinical identifiers in Tracing fields.
- **Check:** feature-coherence across MCP/CLI/client and compatible `rmcp` pin; do not bump versions without protocol interop validation.

## Evidence and release gate
1. Discover actual source, package boundary, MSRV, lockfile, existing tests and Conductor ownership; avoid speculative dependencies.
2. For runtime behaviour, write failing contract tests before implementation, run targeted and full suites; preserve stable schemas, errors, CLI semantics and deterministic outputs.
3. Record `cargo tree -d`, feature selections, security/licensing/advisory reviews, transitive dependency cost and rollback before accepting an adoption.
4. For performance changes, benchmark identical inputs and correctness/invariance oracles; do not claim improvements without measurements.
5. This document is a planning entry, NOT proof of compilation, hosted CI or scientific validation; update status only after a revision-bound receipt.
