# SBOM and artifact attestations

Release artifacts are built by `.github/workflows/release.yml`. Binary archives
have SHA-256 sidecars and existing build provenance attestations.

The release validation job generates `healthpoint-rs.dependencies.cdx.json` from
the exact source Cargo.lock, including resolved transitive dependencies. Grype
enforces the initial critical threshold before binary builds or either publishing
job can proceed. The CycloneDX file is retained as a standalone release asset.

`cargo-metadata.json` inside binary archives is diagnostic Cargo metadata, not a
standard SBOM. The new dependency inventory describes complete source resolution,
including dev/build/optional/target packages, rather than proving the composition
of each platform binary. OS dependencies and independently packaged MCPB/container
runtimes require their own inventories. See [Grype coverage](grype.md).

Signing and vulnerability scanning remain separate from Healthpoint data/licensing
approval. Hosted execution of the new gate is required before release acceptance.

```bash
gh release view <tag> --json assets
shasum -a 256 -c <artifact>.sha256
```
