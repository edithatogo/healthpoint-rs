# Grype dependency gate — 20 November 2026

Track 08 release/distribution maintenance. The release validation job generates a
CycloneDX inventory from the exact source Cargo.lock, gates critical findings
before its dependent build and publishing jobs, and retains the inventory as a
release asset. Existing cargo-metadata snapshots remain diagnostic metadata.
The source advisory workflow covers dependency PRs, relevant main changes and an
explicit manual ref. Weekly checks rescan the retained released SBOM by verified
asset identity; old releases without that asset remain explicit failures until a
new qualified release is published.

Manual publication also refuses to overwrite an existing version's assets from a
different source commit. The guard peels the observed immutable tag object and
distinguishes a genuinely absent tag from a failed lookup; regression cases cover
matching/absent/mismatched tags and lookup errors.

Local workflow/lock-inventory validation is recorded in the scoped PR. Hosted
Grype and full release execution remain pending; this checkpoint does not change
Healthpoint licensing, live-data permission, endpoint or registry readiness.
