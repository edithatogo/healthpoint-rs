# Release provenance plan

Release artifacts must be reproducible enough to audit and must not contain licensed Healthpoint payloads.

## Inputs

- Git tag in the `v*` namespace.
- Committed `Cargo.lock`.
- Pinned RMCP dependency revision.
- GitHub Actions workflow logs for CI and release builds.

## Required checks

Run before tagging:

```bash
bin/conductor-setup
cargo fmt --all --check
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo deny check
```

## Artifact record

For each release, record:

- Git tag and commit SHA.
- Rust toolchain channel and target triples.
- Artifact names and SHA-256 checksums.
- Whether artifacts are unsigned or signed.
- Confirmation that no real Healthpoint payloads, API keys, exports, traces, logs, or caches are included.

## Current signing posture

Initial artifacts are unsigned GitHub Actions build artifacts. Do not describe them as notarized, signed, attested, or supply-chain complete until signing and attestation are implemented.

## Future hardening

- Add checksums to release uploads.
- Add GitHub artifact attestations or Sigstore signing.
- Add macOS notarization if distributing outside local developer use.
- Add SBOM generation after dependency policy is stable.
